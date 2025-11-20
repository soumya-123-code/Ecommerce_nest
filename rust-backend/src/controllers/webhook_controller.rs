use axum::{
    extract::{State, Json},
    http::HeaderMap,
};
use diesel::prelude::*;

use crate::controllers::AppState;
use crate::dto::*;
use crate::models::*;
use crate::schema::*;
use crate::services::OrderService;
use crate::utils::errors::{AppError, AppResult};

/// POST /api/webhooks/stripe
pub async fn stripe_webhook(
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Verify webhook signature
    let _signature = headers
        .get("stripe-signature")
        .ok_or_else(|| AppError::BadRequest("Missing signature".to_string()))?;

    // Parse event
    let event: StripeWebhookPayload = serde_json::from_str(&body)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    match event.event_type.as_str() {
        "payment_intent.succeeded" => {
            if let Some(order_id) = event.data.get("metadata")
                .and_then(|m| m.get("order_id"))
                .and_then(|id| id.as_str())
                .and_then(|s| s.parse::<i32>().ok())
            {
                // Update order status
                diesel::update(orders::table.find(order_id))
                    .set(orders::status.eq("confirmed"))
                    .execute(&mut conn)?;

                // Update payment status
                diesel::update(payments::table.filter(payments::order_id.eq(order_id)))
                    .set(payments::status.eq("completed"))
                    .execute(&mut conn)?;
            }
        }
        "payment_intent.payment_failed" => {
            if let Some(order_id) = event.data.get("metadata")
                .and_then(|m| m.get("order_id"))
                .and_then(|id| id.as_str())
                .and_then(|s| s.parse::<i32>().ok())
            {
                diesel::update(payments::table.filter(payments::order_id.eq(order_id)))
                    .set(payments::status.eq("failed"))
                    .execute(&mut conn)?;
            }
        }
        _ => {}
    }

    Ok(Json(MessageResponse::new("Webhook processed")))
}

/// POST /api/webhooks/razorpay
pub async fn razorpay_webhook(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let event_type = payload.get("event")
        .and_then(|e| e.as_str())
        .unwrap_or("");

    match event_type {
        "payment.captured" => {
            if let Some(order_id) = payload.get("payload")
                .and_then(|p| p.get("payment"))
                .and_then(|p| p.get("entity"))
                .and_then(|e| e.get("notes"))
                .and_then(|n| n.get("order_id"))
                .and_then(|id| id.as_str())
                .and_then(|s| s.parse::<i32>().ok())
            {
                diesel::update(orders::table.find(order_id))
                    .set(orders::status.eq("confirmed"))
                    .execute(&mut conn)?;

                diesel::update(payments::table.filter(payments::order_id.eq(order_id)))
                    .set(payments::status.eq("completed"))
                    .execute(&mut conn)?;
            }
        }
        "payment.failed" => {
            if let Some(order_id) = payload.get("payload")
                .and_then(|p| p.get("payment"))
                .and_then(|p| p.get("entity"))
                .and_then(|e| e.get("notes"))
                .and_then(|n| n.get("order_id"))
                .and_then(|id| id.as_str())
                .and_then(|s| s.parse::<i32>().ok())
            {
                diesel::update(payments::table.filter(payments::order_id.eq(order_id)))
                    .set(payments::status.eq("failed"))
                    .execute(&mut conn)?;
            }
        }
        _ => {}
    }

    Ok(Json(MessageResponse::new("Webhook processed")))
}

/// POST /api/webhooks/paypal
pub async fn paypal_webhook(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let event_type = payload.get("event_type")
        .and_then(|e| e.as_str())
        .unwrap_or("");

    match event_type {
        "PAYMENT.CAPTURE.COMPLETED" => {
            if let Some(order_id) = payload.get("resource")
                .and_then(|r| r.get("custom_id"))
                .and_then(|id| id.as_str())
                .and_then(|s| s.parse::<i32>().ok())
            {
                diesel::update(orders::table.find(order_id))
                    .set(orders::status.eq("confirmed"))
                    .execute(&mut conn)?;

                diesel::update(payments::table.filter(payments::order_id.eq(order_id)))
                    .set(payments::status.eq("completed"))
                    .execute(&mut conn)?;
            }
        }
        _ => {}
    }

    Ok(Json(MessageResponse::new("Webhook processed")))
}
