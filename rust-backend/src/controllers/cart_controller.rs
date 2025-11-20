use axum::{
    extract::{State, Path, Json},
};
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

// ==================== CART DTOs ====================

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemResponse>,
    pub subtotal: String,
    pub items_count: i32,
}

#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub product_id: i32,
    pub product_name: String,
    pub product_image: Option<String>,
    pub price: String,
    pub quantity: i32,
    pub size: Option<String>,
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: i32,
    pub quantity: i32,
    pub size: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub quantity: i32,
}

// Note: Cart is typically stored in session/Redis.
// This implementation uses a simple in-memory approach via client-side storage.
// For production, implement Redis-based cart storage.

/// POST /api/cart/add
/// Django: orders/views.py - add_to_cart
pub async fn add_to_cart(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<AddToCartRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Verify product exists and is active
    let product = products::table
        .find(payload.product_id)
        .first::<Product>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    if !product.is_active {
        return Err(AppError::BadRequest("Product is not available".to_string()));
    }

    if product.stock < payload.quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    // In production: Add to Redis/session cart
    // For now, return success (client manages cart)
    Ok(Json(MessageResponse::new("Added to cart")))
}

/// POST /api/cart/update/:product_id
/// Django: orders/views.py - update_cart
pub async fn update_cart_item(
    State(state): State<std::sync::Arc<AppState>>,
    Path(product_id): Path<i32>,
    Json(payload): Json<UpdateCartRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Verify product exists
    let product = products::table
        .find(product_id)
        .first::<Product>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    if product.stock < payload.quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    Ok(Json(MessageResponse::new("Cart updated")))
}

/// DELETE /api/cart/remove/:product_id
/// Django: orders/views.py - remove_from_cart
pub async fn remove_from_cart(
    Path(_product_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    // In production: Remove from Redis/session cart
    Ok(Json(MessageResponse::new("Removed from cart")))
}

/// DELETE /api/cart/clear
/// Django: orders/views.py - clear_cart
pub async fn clear_cart() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Cart cleared")))
}

/// POST /api/cart/validate
/// Validate cart items before checkout
pub async fn validate_cart(
    State(state): State<std::sync::Arc<AppState>>,
    Json(items): Json<Vec<CartItem>>,
) -> AppResult<Json<ApiResponse<CartResponse>>> {
    let mut conn = state.pool.get()?;

    let mut cart_items = Vec::new();
    let mut subtotal = Decimal::ZERO;

    for item in items {
        let product = products::table
            .find(item.product_id)
            .first::<Product>(&mut conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", item.product_id)))?;

        if !product.is_active {
            return Err(AppError::BadRequest(format!("{} is not available", product.name)));
        }

        if product.stock < item.quantity {
            return Err(AppError::BadRequest(format!("Insufficient stock for {}", product.name)));
        }

        let price = product.discount_price.unwrap_or(product.price);
        let total = price * Decimal::from(item.quantity);
        subtotal += total;

        cart_items.push(CartItemResponse {
            product_id: product.id,
            product_name: product.name,
            product_image: product.image,
            price: price.to_string(),
            quantity: item.quantity,
            size: item.size,
            total: total.to_string(),
        });
    }

    Ok(Json(ApiResponse::success(CartResponse {
        items: cart_items,
        subtotal: subtotal.to_string(),
        items_count: cart_items.len() as i32,
    })))
}

// ==================== VENDOR STATISTICS ====================

#[derive(Debug, Serialize)]
pub struct VendorStatsResponse {
    pub total_products: i64,
    pub total_orders: i64,
    pub total_revenue: String,
    pub pending_orders: i64,
    pub completed_orders: i64,
}

/// GET /api/vendor/stats
/// Django: supplier_panel/views.py - supplier_dashboard
pub async fn get_vendor_stats(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<VendorStatsResponse>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    if !profile.is_vendor {
        return Err(AppError::Forbidden("Vendor access required".to_string()));
    }

    let total_products = products::table
        .filter(products::vendor_id.eq(profile.id))
        .count()
        .get_result::<i64>(&mut conn)?;

    let vendor_orders = order_suppliers::table
        .filter(order_suppliers::vendor_id.eq(profile.id))
        .load::<OrderSupplier>(&mut conn)?;

    let total_orders = vendor_orders.len() as i64;
    let pending_orders = vendor_orders.iter().filter(|o| o.status == "pending").count() as i64;
    let completed_orders = vendor_orders.iter().filter(|o| o.status == "delivered").count() as i64;

    let total_revenue: Decimal = vendor_orders.iter().map(|o| o.payout_amount).sum();

    Ok(Json(ApiResponse::success(VendorStatsResponse {
        total_products,
        total_orders,
        total_revenue: total_revenue.to_string(),
        pending_orders,
        completed_orders,
    })))
}

// ==================== PRODUCT MANAGEMENT ====================

/// DELETE /api/vendor/products/:id
/// Django: supplier_panel/views.py - supplier_delete_product
pub async fn delete_vendor_product(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(product_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    if !profile.is_vendor {
        return Err(AppError::Forbidden("Vendor access required".to_string()));
    }

    // Verify product belongs to vendor
    let product = products::table
        .find(product_id)
        .first::<Product>(&mut conn)?;

    if product.vendor_id != profile.id {
        return Err(AppError::Forbidden("Not your product".to_string()));
    }

    // Soft delete - set inactive
    diesel::update(products::table.find(product_id))
        .set(products::is_active.eq(false))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Product deleted")))
}

// ==================== ORDER TRACKING ====================

/// GET /api/orders/:id/track
/// Django: orders/views.py - track_order
pub async fn track_order(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
) -> AppResult<Json<ApiResponse<OrderTrackingResponse>>> {
    let mut conn = state.pool.get()?;

    let order = orders::table
        .filter(orders::id.eq(order_id))
        .filter(orders::user_id.eq(auth_user.user_id))
        .first::<Order>(&mut conn)
        .map_err(|_| AppError::NotFound("Order not found".to_string()))?;

    let suppliers = order_suppliers::table
        .filter(order_suppliers::order_id.eq(order_id))
        .load::<OrderSupplier>(&mut conn)?;

    let tracking_items: Vec<OrderSupplierTracking> = suppliers
        .into_iter()
        .map(|s| OrderSupplierTracking {
            vendor_id: s.vendor_id,
            status: s.status,
            tracking_number: s.tracking_number,
            shipped_at: s.shipped_at.map(|t| t.to_string()),
            delivered_at: s.delivered_at.map(|t| t.to_string()),
        })
        .collect();

    Ok(Json(ApiResponse::success(OrderTrackingResponse {
        order_number: order.order_number,
        status: order.status,
        suppliers: tracking_items,
        created_at: order.created_at.to_string(),
    })))
}

#[derive(Debug, Serialize)]
pub struct OrderTrackingResponse {
    pub order_number: String,
    pub status: String,
    pub suppliers: Vec<OrderSupplierTracking>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct OrderSupplierTracking {
    pub vendor_id: i32,
    pub status: String,
    pub tracking_number: Option<String>,
    pub shipped_at: Option<String>,
    pub delivered_at: Option<String>,
}

// ==================== USER MANAGEMENT ====================

/// GET /api/users (admin only)
pub async fn list_users(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<User>>>> {
    let mut conn = state.pool.get()?;

    // Check if admin
    let user = users::table.find(auth_user.user_id).first::<User>(&mut conn)?;
    if !user.is_staff {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let user_list = users::table
        .order(users::date_joined.desc())
        .load::<User>(&mut conn)?;

    Ok(Json(ApiResponse::success(user_list)))
}

/// GET /api/users/:id (admin only)
pub async fn get_user(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(user_id): Path<i32>,
) -> AppResult<Json<ApiResponse<User>>> {
    let mut conn = state.pool.get()?;

    // Check if admin
    let admin = users::table.find(auth_user.user_id).first::<User>(&mut conn)?;
    if !admin.is_staff {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)
        .map_err(|_| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(ApiResponse::success(user)))
}

// ==================== VENDOR APPROVAL ====================

/// POST /api/admin/vendors/:id/approve
pub async fn approve_vendor(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(vendor_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Check if admin
    let admin = users::table.find(auth_user.user_id).first::<User>(&mut conn)?;
    if !admin.is_staff {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    diesel::update(profiles::table.find(vendor_id))
        .set(profiles::vendor_admission.eq(true))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Vendor approved")))
}

/// POST /api/admin/vendors/:id/reject
pub async fn reject_vendor(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(vendor_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Check if admin
    let admin = users::table.find(auth_user.user_id).first::<User>(&mut conn)?;
    if !admin.is_staff {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    diesel::update(profiles::table.find(vendor_id))
        .set((
            profiles::is_vendor.eq(false),
            profiles::vendor_admission.eq(false),
        ))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Vendor rejected")))
}

// ==================== POST REPORTS ====================

/// GET /api/blog/posts/:id/reports
pub async fn get_post_reports(
    State(state): State<std::sync::Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<PostReport>>>> {
    let mut conn = state.pool.get()?;

    let reports = post_reports::table
        .filter(post_reports::post_id.eq(post_id))
        .order(post_reports::date.desc())
        .load::<PostReport>(&mut conn)?;

    Ok(Json(ApiResponse::success(reports)))
}

// ==================== WISHLIST ====================

#[derive(Debug, Deserialize)]
pub struct WishlistRequest {
    pub product_id: i32,
}

/// POST /api/wishlist/add
pub async fn add_to_wishlist(
    State(_state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(_payload): Json<WishlistRequest>,
) -> AppResult<Json<MessageResponse>> {
    // Wishlist would need a separate table
    // For now, return success
    Ok(Json(MessageResponse::new("Added to wishlist")))
}

/// DELETE /api/wishlist/:product_id
pub async fn remove_from_wishlist(
    State(_state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(_product_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Removed from wishlist")))
}

/// GET /api/wishlist
pub async fn get_wishlist(
    State(_state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<ProductListResponse>>>> {
    // Return empty for now - would need wishlist table
    Ok(Json(ApiResponse::success(vec![])))
}
