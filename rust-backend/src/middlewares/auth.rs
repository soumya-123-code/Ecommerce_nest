use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::controllers::AppState;
use crate::services::AuthService;

/// Authenticated user extracted from JWT token
/// Django: middleware/auth.py - AuthenticationMiddleware
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
}

#[async_trait]
impl FromRequestParts<std::sync::Arc<AppState>> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &std::sync::Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Extract token from Authorization header
        let token = extract_token(&parts.headers)?;

        // Verify token
        let claims = AuthService::verify_token(&token, &state.config.jwt.secret)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthUser {
            user_id: claims.sub,
            username: claims.username,
        })
    }
}

fn extract_token(headers: &HeaderMap) -> Result<String, AuthError> {
    let header_value = headers
        .get("Authorization")
        .ok_or(AuthError::MissingToken)?
        .to_str()
        .map_err(|_| AuthError::InvalidToken)?;

    if !header_value.starts_with("Bearer ") {
        return Err(AuthError::InvalidToken);
    }

    Ok(header_value[7..].to_string())
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };

        let body = Json(json!({
            "success": false,
            "error": message
        }));

        (status, body).into_response()
    }
}

/// Optional authentication - doesn't fail if no token
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

#[async_trait]
impl FromRequestParts<std::sync::Arc<AppState>> for OptionalAuthUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &std::sync::Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await.ok();
        Ok(OptionalAuthUser(auth_user))
    }
}
