use axum::{
    extract::{State, Multipart, Path},
    Json,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

// ==================== FILE UPLOAD ====================

/// POST /api/upload/image
/// Generic image upload endpoint
pub async fn upload_image(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<UploadResponse>>> {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let file_name = field.file_name()
            .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?
            .to_string();

        let content_type = field.content_type()
            .ok_or_else(|| AppError::BadRequest("No content type".to_string()))?
            .to_string();

        // Validate image type
        if !content_type.starts_with("image/") {
            return Err(AppError::BadRequest("Only image files are allowed".to_string()));
        }

        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

        // Generate unique filename
        let ext = file_name.rsplit('.').next().unwrap_or("jpg");
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let file_path = format!("{}/images/{}", upload_dir, new_filename);

        // Ensure directory exists
        tokio::fs::create_dir_all(format!("{}/images", upload_dir)).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Save file
        tokio::fs::write(&file_path, &data).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let url = format!("/uploads/images/{}", new_filename);

        return Ok(Json(ApiResponse::success(UploadResponse {
            url,
            filename: new_filename,
        })));
    }

    Err(AppError::BadRequest("No file provided".to_string()))
}

/// POST /api/products/:id/images
/// Upload product image
pub async fn upload_product_image(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(product_id): Path<i32>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ProductImageResponse>>> {
    let mut conn = state.pool.get()?;

    // Get user profile
    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    // Verify product belongs to vendor
    let product = products::table
        .find(product_id)
        .first::<Product>(&mut conn)
        .map_err(|_| AppError::NotFound("Product not found".to_string()))?;

    if product.vendor_id != profile.id && !profile.is_vendor {
        return Err(AppError::Forbidden("Not authorized".to_string()));
    }

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let file_name = field.file_name()
            .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?
            .to_string();

        let content_type = field.content_type()
            .ok_or_else(|| AppError::BadRequest("No content type".to_string()))?
            .to_string();

        if !content_type.starts_with("image/") {
            return Err(AppError::BadRequest("Only image files are allowed".to_string()));
        }

        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

        let ext = file_name.rsplit('.').next().unwrap_or("jpg");
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let file_path = format!("{}/products/{}", upload_dir, new_filename);

        tokio::fs::create_dir_all(format!("{}/products", upload_dir)).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        tokio::fs::write(&file_path, &data).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let url = format!("/uploads/products/{}", new_filename);

        // Get current max order
        let max_order: Option<i32> = product_images::table
            .filter(product_images::product_id.eq(product_id))
            .select(diesel::dsl::max(product_images::order))
            .first(&mut conn)?;

        // Insert into database
        let new_image = NewProductImage {
            product_id,
            image: url.clone(),
            order: max_order.unwrap_or(0) + 1,
        };

        diesel::insert_into(product_images::table)
            .values(&new_image)
            .execute(&mut conn)?;

        let image = product_images::table
            .filter(product_images::product_id.eq(product_id))
            .filter(product_images::image.eq(&url))
            .first::<ProductImage>(&mut conn)?;

        return Ok(Json(ApiResponse::success(ProductImageResponse {
            id: image.id,
            image: image.image,
            order: image.order,
        })));
    }

    Err(AppError::BadRequest("No file provided".to_string()))
}

/// DELETE /api/products/:product_id/images/:image_id
/// Delete product image
pub async fn delete_product_image(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path((product_id, image_id)): Path<(i32, i32)>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    let product = products::table
        .find(product_id)
        .first::<Product>(&mut conn)
        .map_err(|_| AppError::NotFound("Product not found".to_string()))?;

    if product.vendor_id != profile.id {
        return Err(AppError::Forbidden("Not authorized".to_string()));
    }

    diesel::delete(product_images::table
        .filter(product_images::id.eq(image_id))
        .filter(product_images::product_id.eq(product_id)))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Image deleted")))
}

/// POST /api/auth/avatar
/// Upload user avatar
pub async fn upload_avatar(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<UploadResponse>>> {
    let mut conn = state.pool.get()?;
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let file_name = field.file_name()
            .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?
            .to_string();

        let content_type = field.content_type()
            .ok_or_else(|| AppError::BadRequest("No content type".to_string()))?
            .to_string();

        if !content_type.starts_with("image/") {
            return Err(AppError::BadRequest("Only image files are allowed".to_string()));
        }

        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

        let ext = file_name.rsplit('.').next().unwrap_or("jpg");
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let file_path = format!("{}/avatars/{}", upload_dir, new_filename);

        tokio::fs::create_dir_all(format!("{}/avatars", upload_dir)).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        tokio::fs::write(&file_path, &data).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let url = format!("/uploads/avatars/{}", new_filename);

        // Update profile
        diesel::update(profiles::table.filter(profiles::user_id.eq(auth_user.user_id)))
            .set(profiles::avatar.eq(&url))
            .execute(&mut conn)?;

        return Ok(Json(ApiResponse::success(UploadResponse {
            url,
            filename: new_filename,
        })));
    }

    Err(AppError::BadRequest("No file provided".to_string()))
}

// ==================== DTOs ====================

#[derive(Debug, serde::Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub filename: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ProductImageResponse {
    pub id: i32,
    pub image: String,
    pub order: i32,
}
