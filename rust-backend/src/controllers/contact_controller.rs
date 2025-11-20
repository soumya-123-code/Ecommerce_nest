use axum::{
    extract::{State, Json},
};
use diesel::prelude::*;

use crate::controllers::AppState;
use crate::dto::*;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::AppResult;

/// POST /api/newsletter/subscribe
pub async fn subscribe_newsletter(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<NewsletterRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Check if already subscribed
    let existing = newsletters::table
        .filter(newsletters::email.eq(&payload.email))
        .first::<Newsletter>(&mut conn)
        .optional()?;

    if existing.is_some() {
        return Ok(Json(MessageResponse::new("Already subscribed")));
    }

    let new_subscription = NewNewsletter {
        email: payload.email,
    };

    diesel::insert_into(newsletters::table)
        .values(&new_subscription)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Successfully subscribed to newsletter")))
}

/// POST /api/newsletter/unsubscribe
pub async fn unsubscribe_newsletter(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<NewsletterRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    diesel::update(newsletters::table.filter(newsletters::email.eq(&payload.email)))
        .set(newsletters::is_active.eq(false))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Successfully unsubscribed")))
}

/// POST /api/contact
pub async fn submit_contact(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<ContactRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let new_message = NewContactMessage {
        name: payload.name,
        email: payload.email,
        subject: payload.subject,
        message: payload.message,
    };

    diesel::insert_into(contact_messages::table)
        .values(&new_message)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Message sent successfully")))
}
