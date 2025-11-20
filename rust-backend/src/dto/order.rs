use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ==================== Order DTOs ====================
// Django: orders/serializers.py → Rust DTOs

#[derive(Debug, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: i32,
    pub size: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub items: Vec<CartItem>,
    #[validate(length(min = 10))]
    pub shipping_address: String,
    pub billing_address: Option<String>,
    #[validate(length(min = 5))]
    pub phone: String,
    #[validate(email)]
    pub email: String,
    pub notes: Option<String>,
    pub coupon_code: Option<String>,
    pub payment_method: String,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub order_number: String,
    pub status: String,
    pub items: Vec<OrderItemResponse>,
    pub subtotal: String,
    pub shipping_cost: String,
    pub tax: String,
    pub discount: String,
    pub total: String,
    pub shipping_address: String,
    pub billing_address: String,
    pub phone: String,
    pub email: String,
    pub notes: Option<String>,
    pub payment: Option<PaymentInfo>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub product_image: Option<String>,
    pub size: Option<String>,
    pub quantity: i32,
    pub unit_price: String,
    pub total_price: String,
}

#[derive(Debug, Serialize)]
pub struct OrderListResponse {
    pub id: i32,
    pub order_number: String,
    pub status: String,
    pub total: String,
    pub items_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentInfo {
    pub id: i32,
    pub method: String,
    pub transaction_id: Option<String>,
    pub amount: String,
    pub status: String,
}

// ==================== Vendor Order DTOs ====================

#[derive(Debug, Serialize)]
pub struct VendorOrderResponse {
    pub id: i32,
    pub order_id: i32,
    pub order_number: String,
    pub customer_name: String,
    pub status: String,
    pub items: Vec<OrderItemResponse>,
    pub subtotal: String,
    pub commission_amount: String,
    pub payout_amount: String,
    pub tracking_number: Option<String>,
    pub shipped_at: Option<String>,
    pub delivered_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVendorOrderRequest {
    pub status: Option<String>,
    pub tracking_number: Option<String>,
}

// ==================== Payment DTOs ====================

#[derive(Debug, Deserialize)]
pub struct InitiatePaymentRequest {
    pub order_id: i32,
    pub payment_method: String,
    pub return_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: Option<String>,
    pub payment_url: Option<String>,
    pub order_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookPayload {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: serde_json::Value,
}

// ==================== Coupon DTOs ====================

#[derive(Debug, Deserialize)]
pub struct ApplyCouponRequest {
    pub code: String,
    pub subtotal: Decimal,
}

#[derive(Debug, Serialize)]
pub struct CouponResponse {
    pub valid: bool,
    pub discount_type: String,
    pub discount_value: String,
    pub discount_amount: String,
    pub message: Option<String>,
}

// ==================== Vendor Payment Request ====================

#[derive(Debug, Deserialize, Validate)]
pub struct VendorPaymentRequest {
    #[validate(range(min = 1.0))]
    pub amount: Decimal,
    pub bank_account_id: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VendorWalletResponse {
    pub balance: String,
    pub pending_payments: String,
    pub total_earned: String,
    pub recent_transactions: Vec<VendorTransactionResponse>,
}

#[derive(Debug, Serialize)]
pub struct VendorTransactionResponse {
    pub id: i32,
    pub amount: String,
    pub payment_type: String,
    pub status: String,
    pub reference: Option<String>,
    pub created_at: String,
}
