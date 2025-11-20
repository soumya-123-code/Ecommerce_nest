use axum::{
    extract::{State, Path, Json},
};

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::services::{AuthService, ProductService, OrderService, UserService};
use crate::utils::errors::{AppError, AppResult};

/// Middleware check for vendor status
async fn check_vendor(
    state: &std::sync::Arc<AppState>,
    user_id: i32,
) -> AppResult<i32> {
    let mut conn = state.pool.get()?;
    let profile = AuthService::get_profile(&mut conn, user_id)?;

    if !profile.is_vendor || !profile.vendor_admission {
        return Err(AppError::Forbidden("Vendor access required".to_string()));
    }

    Ok(profile.id)
}

/// GET /api/vendor/products
/// Django: supplier_panel/views.py - supplier_products
pub async fn list_vendor_products(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<ProductListResponse>>>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    let products = UserService::get_vendor_products(&mut conn, vendor_id)?;

    let response: Vec<ProductListResponse> = products
        .into_iter()
        .map(|p| ProductListResponse {
            id: p.id,
            name: p.name,
            slug: p.slug,
            price: p.price.to_string(),
            discount_price: p.discount_price.map(|d| d.to_string()),
            image: p.image,
            rating: 0.0,
            reviews_count: 0,
            is_featured: p.is_featured,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/vendor/products
/// Django: supplier_panel/views.py - supplier_add_product
pub async fn create_vendor_product(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<CreateProductRequest>,
) -> AppResult<Json<ApiResponse<ProductListResponse>>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    let product = ProductService::create_product(
        &mut conn,
        vendor_id,
        &payload.name,
        payload.mini_category_id,
        &payload.description,
        payload.price,
        payload.discount_price,
        payload.stock,
        payload.sku,
    )?;

    Ok(Json(ApiResponse::success(ProductListResponse {
        id: product.id,
        name: product.name,
        slug: product.slug,
        price: product.price.to_string(),
        discount_price: product.discount_price.map(|d| d.to_string()),
        image: product.image,
        rating: 0.0,
        reviews_count: 0,
        is_featured: product.is_featured,
    })))
}

/// PUT /api/vendor/products/:id
/// Django: supplier_panel/views.py - supplier_edit_product
pub async fn update_vendor_product(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(product_id): Path<i32>,
    Json(payload): Json<UpdateProductRequest>,
) -> AppResult<Json<MessageResponse>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    use crate::models::UpdateProduct;

    let updates = UpdateProduct {
        name: payload.name,
        slug: None,
        description: payload.description,
        price: payload.price,
        discount_price: payload.discount_price,
        stock: payload.stock,
        sku: payload.sku,
        image: None,
        is_active: payload.is_active,
        is_featured: payload.is_featured,
    };

    ProductService::update_product(&mut conn, product_id, vendor_id, updates)?;

    Ok(Json(MessageResponse::new("Product updated successfully")))
}

/// GET /api/vendor/orders
/// Django: supplier_panel/views.py - supplier_orders
pub async fn list_vendor_orders(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<VendorOrderResponse>>>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    let orders = OrderService::get_vendor_orders(&mut conn, vendor_id)?;

    let response: Vec<VendorOrderResponse> = orders
        .into_iter()
        .map(|o| VendorOrderResponse {
            id: o.id,
            order_id: o.order_id,
            order_number: String::new(), // Would need to join with orders
            customer_name: String::new(),
            status: o.status,
            items: vec![],
            subtotal: o.subtotal.to_string(),
            commission_amount: o.commission_amount.to_string(),
            payout_amount: o.payout_amount.to_string(),
            tracking_number: o.tracking_number,
            shipped_at: o.shipped_at.map(|t| t.to_string()),
            delivered_at: o.delivered_at.map(|t| t.to_string()),
            created_at: String::new(),
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// PUT /api/vendor/orders/:id
/// Django: supplier_panel/views.py - supplier_order_update
pub async fn update_vendor_order(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
    Json(payload): Json<UpdateVendorOrderRequest>,
) -> AppResult<Json<MessageResponse>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    OrderService::update_vendor_order(
        &mut conn,
        order_id,
        vendor_id,
        payload.status.as_deref().unwrap_or("pending"),
        payload.tracking_number,
    )?;

    Ok(Json(MessageResponse::new("Order updated successfully")))
}

/// GET /api/vendor/wallet
/// Django: supplier_panel/views.py - supplier_wallet
pub async fn get_vendor_wallet(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<VendorWalletResponse>>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    let profile = AuthService::get_profile(&mut conn, auth_user.user_id)?;
    let payments = UserService::get_vendor_payments(&mut conn, vendor_id)?;

    let transactions: Vec<VendorTransactionResponse> = payments
        .iter()
        .take(10)
        .map(|p| VendorTransactionResponse {
            id: p.id,
            amount: p.amount.to_string(),
            payment_type: p.payment_type.clone(),
            status: p.status.clone(),
            reference: p.reference_number.clone(),
            created_at: p.created_at.to_string(),
        })
        .collect();

    Ok(Json(ApiResponse::success(VendorWalletResponse {
        balance: profile.wallet_balance.to_string(),
        pending_payments: "0".to_string(),
        total_earned: "0".to_string(),
        recent_transactions: transactions,
    })))
}

/// POST /api/vendor/payments/request
/// Django: supplier_panel/views.py - supplier_request_payment
pub async fn request_vendor_payout(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<VendorPaymentRequest>,
) -> AppResult<Json<MessageResponse>> {
    let vendor_id = check_vendor(&state, auth_user.user_id).await?;
    let mut conn = state.pool.get()?;

    UserService::request_payout(
        &mut conn,
        vendor_id,
        payload.amount,
        payload.bank_account_id,
        payload.notes,
    )?;

    Ok(Json(MessageResponse::new("Payout request submitted")))
}
