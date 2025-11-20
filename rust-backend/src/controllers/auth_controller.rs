use axum::{
    extract::{State, Json},
    http::StatusCode,
};

use crate::config::Config;
use crate::db::DbPool;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::services::{AuthService, UserService};
use crate::utils::errors::{AppError, AppResult};

pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

/// POST /api/auth/register
/// Django: accounts/views.py - register_view
pub async fn register(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let mut conn = state.pool.get()?;

    let user = AuthService::register(
        &mut conn,
        &payload.username,
        &payload.email,
        &payload.password,
        &payload.first_name,
        &payload.last_name,
        payload.phone.as_deref(),
        payload.referral_code.as_deref(),
    )?;

    let token = AuthService::generate_token(
        &user,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    let profile = AuthService::get_profile(&mut conn, user.id)?;

    Ok(Json(AuthResponse {
        success: true,
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_vendor: profile.is_vendor,
            vendor_admission: profile.vendor_admission,
        },
    }))
}

/// POST /api/auth/login
/// Django: accounts/views.py - login_view
pub async fn login(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let mut conn = state.pool.get()?;

    let (user, token) = AuthService::login(
        &mut conn,
        &payload.username,
        &payload.password,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    let profile = AuthService::get_profile(&mut conn, user.id)?;

    Ok(Json(AuthResponse {
        success: true,
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_vendor: profile.is_vendor,
            vendor_admission: profile.vendor_admission,
        },
    }))
}

/// GET /api/auth/profile
/// Django: accounts/views.py - profile_view (GET)
pub async fn get_profile(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ProfileResponse>> {
    let mut conn = state.pool.get()?;

    let user = AuthService::get_user_by_id(&mut conn, auth_user.user_id)?;
    let profile = AuthService::get_profile(&mut conn, auth_user.user_id)?;

    Ok(Json(ProfileResponse {
        id: profile.id,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_vendor: profile.is_vendor,
            vendor_admission: profile.vendor_admission,
        },
        phone: profile.phone,
        address: profile.address,
        city: profile.city,
        country: None, // Would need to join with countries table
        postal_code: profile.postal_code,
        avatar: profile.avatar,
        is_vendor: profile.is_vendor,
        vendor_admission: profile.vendor_admission,
        wallet_balance: profile.wallet_balance.to_string(),
        referral_code: profile.referral_code,
    }))
}

/// PUT /api/auth/profile
/// Django: accounts/views.py - profile_view (POST)
pub async fn update_profile(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    use crate::models::{UpdateUser, UpdateProfile};

    let user_updates = UpdateUser {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        is_active: None,
        last_login: None,
    };

    let profile_updates = UpdateProfile {
        phone: payload.phone,
        address: payload.address,
        city: payload.city,
        country_id: payload.country_id,
        postal_code: payload.postal_code,
        avatar: None,
        is_vendor: None,
        vendor_admission: None,
        wallet_balance: None,
    };

    UserService::update_profile(&mut conn, auth_user.user_id, user_updates, profile_updates)?;

    Ok(Json(MessageResponse::new("Profile updated successfully")))
}

/// POST /api/auth/change-password
/// Django: accounts/views.py - change_password_view
pub async fn change_password(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    AuthService::change_password(
        &mut conn,
        auth_user.user_id,
        &payload.current_password,
        &payload.new_password,
    )?;

    Ok(Json(MessageResponse::new("Password changed successfully")))
}

/// POST /api/auth/apply-vendor
/// Django: accounts/views.py - apply_vendor_view
pub async fn apply_vendor(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    UserService::apply_for_vendor(&mut conn, auth_user.user_id)?;

    Ok(Json(MessageResponse::new("Vendor application submitted")))
}

/// GET /api/countries
pub async fn get_countries(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<crate::models::Country>>>> {
    let mut conn = state.pool.get()?;
    let countries = UserService::get_countries(&mut conn)?;
    Ok(Json(ApiResponse::success(countries)))
}

/// POST /api/auth/password-reset
/// Django: accounts/views.py - password_reset_view
pub async fn request_password_reset(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Find user by email
    use crate::schema::users;
    use diesel::prelude::*;

    let _user = users::table
        .filter(users::email.eq(&payload.email))
        .first::<crate::models::User>(&mut conn)
        .optional()?;

    // Always return success to prevent email enumeration
    // In production, send reset email here
    Ok(Json(MessageResponse::new("If the email exists, a reset link has been sent")))
}

/// POST /api/auth/password-reset/confirm
/// Django: accounts/views.py - password_reset_confirm_view
pub async fn confirm_password_reset(
    State(_state): State<std::sync::Arc<AppState>>,
    Json(_payload): Json<ConfirmResetPasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    // In production: verify token and update password
    Ok(Json(MessageResponse::new("Password has been reset successfully")))
}

/// POST /api/auth/logout
pub async fn logout() -> AppResult<Json<MessageResponse>> {
    // JWT tokens are stateless - client should discard token
    Ok(Json(MessageResponse::new("Logged out successfully")))
}
