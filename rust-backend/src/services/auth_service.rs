use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DbConnection;
use crate::models::{User, NewUser, Profile, NewProfile};
use crate::schema::{users, profiles};
use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,       // user id
    pub username: String,
    pub exp: i64,       // expiration time
    pub iat: i64,       // issued at
}

pub struct AuthService;

impl AuthService {
    /// Register a new user
    /// Django: accounts/views.py - register_view
    pub fn register(
        conn: &mut DbConnection,
        username: &str,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
        phone: Option<&str>,
        referral_code: Option<&str>,
    ) -> AppResult<User> {
        // Check if user exists
        let existing = users::table
            .filter(users::username.eq(username).or(users::email.eq(email)))
            .first::<User>(conn)
            .optional()?;

        if existing.is_some() {
            return Err(AppError::Validation("Username or email already exists".to_string()));
        }

        // Hash password
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Find referrer if code provided
        let referred_by_id = if let Some(code) = referral_code {
            profiles::table
                .filter(profiles::referral_code.eq(code))
                .select(profiles::id)
                .first::<i32>(conn)
                .optional()?
        } else {
            None
        };

        // Create user
        let new_user = NewUser {
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        let user = users::table
            .order(users::id.desc())
            .first::<User>(conn)?;

        // Create profile with referral code
        let new_profile = NewProfile {
            user_id: user.id,
            phone: phone.map(|s| s.to_string()),
            referral_code: Some(generate_referral_code()),
            referred_by_id,
        };

        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;

        Ok(user)
    }

    /// Authenticate user and return token
    /// Django: accounts/views.py - login_view
    pub fn login(
        conn: &mut DbConnection,
        username: &str,
        password: &str,
        jwt_secret: &str,
        jwt_expiration: i64,
    ) -> AppResult<(User, String)> {
        let user = users::table
            .filter(users::username.eq(username).or(users::email.eq(username)))
            .first::<User>(conn)
            .optional()?
            .ok_or_else(|| AppError::Auth("Invalid credentials".to_string()))?;

        if !user.is_active {
            return Err(AppError::Auth("Account is disabled".to_string()));
        }

        let valid = verify(password, &user.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if !valid {
            return Err(AppError::Auth("Invalid credentials".to_string()));
        }

        // Update last login
        diesel::update(users::table.find(user.id))
            .set(users::last_login.eq(Utc::now().naive_utc()))
            .execute(conn)?;

        // Generate JWT token
        let token = Self::generate_token(&user, jwt_secret, jwt_expiration)?;

        Ok((user, token))
    }

    /// Generate JWT token
    pub fn generate_token(user: &User, secret: &str, expiration: i64) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            exp: (now + Duration::seconds(expiration)).timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Jwt(e))
    }

    /// Verify JWT token and return claims
    pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Get user by ID
    pub fn get_user_by_id(conn: &mut DbConnection, user_id: i32) -> AppResult<User> {
        users::table
            .find(user_id)
            .first::<User>(conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Get user profile
    pub fn get_profile(conn: &mut DbConnection, user_id: i32) -> AppResult<Profile> {
        profiles::table
            .filter(profiles::user_id.eq(user_id))
            .first::<Profile>(conn)
            .map_err(|_| AppError::NotFound("Profile not found".to_string()))
    }

    /// Change password
    pub fn change_password(
        conn: &mut DbConnection,
        user_id: i32,
        current_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let user = Self::get_user_by_id(conn, user_id)?;

        let valid = verify(current_password, &user.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if !valid {
            return Err(AppError::Auth("Current password is incorrect".to_string()));
        }

        let new_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        diesel::update(users::table.find(user_id))
            .set(users::password_hash.eq(new_hash))
            .execute(conn)?;

        Ok(())
    }
}

fn generate_referral_code() -> String {
    Uuid::new_v4().to_string()[..8].to_uppercase()
}
