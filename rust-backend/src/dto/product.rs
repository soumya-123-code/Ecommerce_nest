use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ==================== Product DTOs ====================
// Django: products/serializers.py → Rust DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub mini_category_id: i32,
    pub description: String,
    #[validate(range(min = 0.01))]
    pub price: Decimal,
    pub discount_price: Option<Decimal>,
    #[validate(range(min = 0))]
    pub stock: i32,
    pub sku: Option<String>,
    pub sizes: Option<Vec<ProductSizeInput>>,
}

#[derive(Debug, Deserialize)]
pub struct ProductSizeInput {
    pub size: String,
    pub stock: i32,
    pub price_adjustment: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub discount_price: Option<Decimal>,
    pub stock: Option<i32>,
    pub sku: Option<String>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub price: String,
    pub discount_price: Option<String>,
    pub stock: i32,
    pub sku: Option<String>,
    pub image: Option<String>,
    pub images: Vec<ProductImageResponse>,
    pub sizes: Vec<ProductSizeResponse>,
    pub category: CategoryBreadcrumb,
    pub vendor: VendorInfo,
    pub rating: RatingInfo,
    pub is_active: bool,
    pub is_featured: bool,
    pub views_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub price: String,
    pub discount_price: Option<String>,
    pub image: Option<String>,
    pub rating: f64,
    pub reviews_count: i32,
    pub is_featured: bool,
}

#[derive(Debug, Serialize)]
pub struct ProductImageResponse {
    pub id: i32,
    pub image: String,
    pub alt_text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductSizeResponse {
    pub id: i32,
    pub size: String,
    pub stock: i32,
    pub price_adjustment: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryBreadcrumb {
    pub super_category: CategoryInfo,
    pub main_category: CategoryInfo,
    pub sub_category: CategoryInfo,
    pub mini_category: CategoryInfo,
}

#[derive(Debug, Serialize)]
pub struct CategoryInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct VendorInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RatingInfo {
    pub average: f64,
    pub count: i32,
    pub distribution: RatingDistribution,
}

#[derive(Debug, Serialize)]
pub struct RatingDistribution {
    pub five_star: i32,
    pub four_star: i32,
    pub three_star: i32,
    pub two_star: i32,
    pub one_star: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRatingRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryTreeResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
    pub children: Vec<CategoryTreeResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ProductFilterParams {
    pub category: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub sort_by: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
