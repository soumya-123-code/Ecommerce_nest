use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::{
    orders, order_details, order_suppliers, order_details_suppliers,
    coupons, payments, vendor_payments
};

// ==================== Order Status Enum ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Confirmed => "confirmed",
            OrderStatus::Processing => "processing",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Delivered => "delivered",
            OrderStatus::Cancelled => "cancelled",
            OrderStatus::Refunded => "refunded",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "confirmed" => OrderStatus::Confirmed,
            "processing" => OrderStatus::Processing,
            "shipped" => OrderStatus::Shipped,
            "delivered" => OrderStatus::Delivered,
            "cancelled" => OrderStatus::Cancelled,
            "refunded" => OrderStatus::Refunded,
            _ => OrderStatus::Pending,
        }
    }
}

// ==================== Order Model ====================
// Django: orders.Order → Rust: Order struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub order_number: String,
    pub status: String,
    pub subtotal: Decimal,
    pub shipping_cost: Decimal,
    pub tax: Decimal,
    pub discount: Decimal,
    pub total: Decimal,
    pub shipping_address: String,
    pub billing_address: String,
    pub phone: String,
    pub email: String,
    pub notes: Option<String>,
    pub coupon_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub order_number: String,
    pub status: String,
    pub subtotal: Decimal,
    pub shipping_cost: Decimal,
    pub tax: Decimal,
    pub discount: Decimal,
    pub total: Decimal,
    pub shipping_address: String,
    pub billing_address: String,
    pub phone: String,
    pub email: String,
    pub notes: Option<String>,
    pub coupon_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = orders)]
pub struct UpdateOrder {
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ==================== Order Detail Model ====================
// Django: orders.OrderDetails → Rust: OrderDetail struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = order_details)]
pub struct OrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub size: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = order_details)]
pub struct NewOrderDetail {
    pub order_id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub size: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

// ==================== Order Supplier Model ====================
// Django: orders.OrderSupplier → Rust: OrderSupplier struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = order_suppliers)]
pub struct OrderSupplier {
    pub id: i32,
    pub order_id: i32,
    pub vendor_id: i32,
    pub status: String,
    pub subtotal: Decimal,
    pub commission_rate: Decimal,
    pub commission_amount: Decimal,
    pub payout_amount: Decimal,
    pub tracking_number: Option<String>,
    pub shipped_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = order_suppliers)]
pub struct NewOrderSupplier {
    pub order_id: i32,
    pub vendor_id: i32,
    pub status: String,
    pub subtotal: Decimal,
    pub commission_rate: Decimal,
    pub commission_amount: Decimal,
    pub payout_amount: Decimal,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = order_suppliers)]
pub struct UpdateOrderSupplier {
    pub status: Option<String>,
    pub tracking_number: Option<String>,
    pub shipped_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,
}

// ==================== Order Details Supplier Model ====================

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(OrderSupplier))]
#[diesel(table_name = order_details_suppliers)]
pub struct OrderDetailSupplier {
    pub id: i32,
    pub order_supplier_id: i32,
    pub order_detail_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = order_details_suppliers)]
pub struct NewOrderDetailSupplier {
    pub order_supplier_id: i32,
    pub order_detail_id: i32,
}

// ==================== Coupon Model ====================
// Django: orders.Coupon → Rust: Coupon struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = coupons)]
pub struct Coupon {
    pub id: i32,
    pub code: String,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_purchase: Decimal,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub valid_from: NaiveDateTime,
    pub valid_to: NaiveDateTime,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = coupons)]
pub struct NewCoupon {
    pub code: String,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_purchase: Decimal,
    pub max_uses: Option<i32>,
    pub valid_from: NaiveDateTime,
    pub valid_to: NaiveDateTime,
}

// ==================== Payment Model ====================
// Django: orders.Payment → Rust: Payment struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: i32,
    pub order_id: i32,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub gateway_response: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = payments)]
pub struct NewPayment {
    pub order_id: i32,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub gateway_response: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = payments)]
pub struct UpdatePayment {
    pub transaction_id: Option<String>,
    pub status: Option<String>,
    pub gateway_response: Option<String>,
}

// ==================== Vendor Payment Model ====================
// Django: payments.VendorPayments → Rust: VendorPayment struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = vendor_payments)]
pub struct VendorPayment {
    pub id: i32,
    pub vendor_id: i32,
    pub order_supplier_id: Option<i32>,
    pub amount: Decimal,
    pub payment_type: String,
    pub status: String,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub processed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = vendor_payments)]
pub struct NewVendorPayment {
    pub vendor_id: i32,
    pub order_supplier_id: Option<i32>,
    pub amount: Decimal,
    pub payment_type: String,
    pub status: String,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}
