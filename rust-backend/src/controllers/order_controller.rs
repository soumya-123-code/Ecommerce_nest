use axum::{
    extract::{State, Path, Json},
};

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::services::OrderService;
use crate::utils::errors::AppResult;

/// POST /api/orders
/// Django: orders/views.py - checkout_view
pub async fn create_order(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<CreateOrderRequest>,
) -> AppResult<Json<ApiResponse<OrderResponse>>> {
    let mut conn = state.pool.get()?;

    let billing_address = payload.billing_address
        .unwrap_or_else(|| payload.shipping_address.clone());

    let order = OrderService::create_order(
        &mut conn,
        auth_user.user_id,
        &payload.items,
        &payload.shipping_address,
        &billing_address,
        &payload.phone,
        &payload.email,
        payload.notes,
        payload.coupon_code.as_deref(),
    )?;

    let details = OrderService::get_order_details(&mut conn, order.id)?;

    let response = OrderResponse {
        id: order.id,
        order_number: order.order_number,
        status: order.status,
        items: details
            .into_iter()
            .map(|d| OrderItemResponse {
                id: d.id,
                product_id: d.product_id,
                product_name: d.product_name,
                product_image: None,
                size: d.size,
                quantity: d.quantity,
                unit_price: d.unit_price.to_string(),
                total_price: d.total_price.to_string(),
            })
            .collect(),
        subtotal: order.subtotal.to_string(),
        shipping_cost: order.shipping_cost.to_string(),
        tax: order.tax.to_string(),
        discount: order.discount.to_string(),
        total: order.total.to_string(),
        shipping_address: order.shipping_address,
        billing_address: order.billing_address,
        phone: order.phone,
        email: order.email,
        notes: order.notes,
        payment: None,
        created_at: order.created_at.to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/orders
/// Django: orders/views.py - order_history_view
pub async fn list_orders(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<OrderListResponse>>>> {
    let mut conn = state.pool.get()?;

    let orders = OrderService::get_user_orders(&mut conn, auth_user.user_id)?;

    let response: Vec<OrderListResponse> = orders
        .into_iter()
        .map(|o| OrderListResponse {
            id: o.id,
            order_number: o.order_number,
            status: o.status,
            total: o.total.to_string(),
            items_count: 0, // Would need to count from order_details
            created_at: o.created_at.to_string(),
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/orders/:id
/// Django: orders/views.py - order_detail_view
pub async fn get_order(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
) -> AppResult<Json<ApiResponse<OrderResponse>>> {
    let mut conn = state.pool.get()?;

    let order = OrderService::get_order(&mut conn, order_id, auth_user.user_id)?;
    let details = OrderService::get_order_details(&mut conn, order.id)?;

    let response = OrderResponse {
        id: order.id,
        order_number: order.order_number,
        status: order.status,
        items: details
            .into_iter()
            .map(|d| OrderItemResponse {
                id: d.id,
                product_id: d.product_id,
                product_name: d.product_name,
                product_image: None,
                size: d.size,
                quantity: d.quantity,
                unit_price: d.unit_price.to_string(),
                total_price: d.total_price.to_string(),
            })
            .collect(),
        subtotal: order.subtotal.to_string(),
        shipping_cost: order.shipping_cost.to_string(),
        tax: order.tax.to_string(),
        discount: order.discount.to_string(),
        total: order.total.to_string(),
        shipping_address: order.shipping_address,
        billing_address: order.billing_address,
        phone: order.phone,
        email: order.email,
        notes: order.notes,
        payment: None,
        created_at: order.created_at.to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/payments/initiate
/// Django: orders/views.py - initiate_payment
pub async fn initiate_payment(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<InitiatePaymentRequest>,
) -> AppResult<Json<ApiResponse<PaymentIntentResponse>>> {
    let mut conn = state.pool.get()?;

    // Verify order belongs to user
    let order = OrderService::get_order(&mut conn, payload.order_id, auth_user.user_id)?;

    // Create payment record
    let _payment = OrderService::create_payment(
        &mut conn,
        order.id,
        &payload.payment_method,
        None,
        order.total,
        "USD",
        "pending",
    )?;

    // Would integrate with Stripe/PayPal here
    let response = PaymentIntentResponse {
        client_secret: Some("pi_xxx_secret_xxx".to_string()),
        payment_url: None,
        order_id: order.id.to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/coupons/apply
/// Django: orders/views.py - apply_coupon_view
pub async fn apply_coupon(
    State(_state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<ApplyCouponRequest>,
) -> AppResult<Json<CouponResponse>> {
    // Would validate coupon here
    Ok(Json(CouponResponse {
        valid: true,
        discount_type: "percentage".to_string(),
        discount_value: "10".to_string(),
        discount_amount: (payload.subtotal * rust_decimal::Decimal::new(10, 2)).to_string(),
        message: None,
    }))
}
