use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::{
    super_categories, main_categories, sub_categories, mini_categories,
    products, product_images, product_sizes, product_ratings
};

// ==================== Category Models ====================
// Django: products.SuperCategory → Rust: SuperCategory struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = super_categories)]
pub struct SuperCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = super_categories)]
pub struct NewSuperCategory {
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(SuperCategory))]
#[diesel(table_name = main_categories)]
pub struct MainCategory {
    pub id: i32,
    pub super_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = main_categories)]
pub struct NewMainCategory {
    pub super_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(MainCategory))]
#[diesel(table_name = sub_categories)]
pub struct SubCategory {
    pub id: i32,
    pub main_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = sub_categories)]
pub struct NewSubCategory {
    pub main_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(SubCategory))]
#[diesel(table_name = mini_categories)]
pub struct MiniCategory {
    pub id: i32,
    pub sub_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = mini_categories)]
pub struct NewMiniCategory {
    pub sub_category_id: i32,
    pub name: String,
    pub slug: String,
    pub image: Option<String>,
}

// ==================== Product Model ====================
// Django: products.Product → Rust: Product struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(MiniCategory))]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub vendor_id: i32,
    pub mini_category_id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub price: Decimal,
    pub discount_price: Option<Decimal>,
    pub stock: i32,
    pub sku: Option<String>,
    pub image: Option<String>,
    pub is_active: bool,
    pub is_featured: bool,
    pub views_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub vendor_id: i32,
    pub mini_category_id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub price: Decimal,
    pub discount_price: Option<Decimal>,
    pub stock: i32,
    pub sku: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub discount_price: Option<Decimal>,
    pub stock: Option<i32>,
    pub sku: Option<String>,
    pub image: Option<String>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
}

// ==================== Product Image Model ====================

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_images)]
pub struct ProductImage {
    pub id: i32,
    pub product_id: i32,
    pub image: String,
    pub alt_text: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = product_images)]
pub struct NewProductImage {
    pub product_id: i32,
    pub image: String,
    pub alt_text: Option<String>,
    pub sort_order: i32,
}

// ==================== Product Size Model ====================

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_sizes)]
pub struct ProductSize {
    pub id: i32,
    pub product_id: i32,
    pub size: String,
    pub stock: i32,
    pub price_adjustment: Decimal,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = product_sizes)]
pub struct NewProductSize {
    pub product_id: i32,
    pub size: String,
    pub stock: i32,
    pub price_adjustment: Decimal,
}

// ==================== Product Rating Model ====================
// Django: products.ProductRating → Rust: ProductRating struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_ratings)]
pub struct ProductRating {
    pub id: i32,
    pub product_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = product_ratings)]
pub struct NewProductRating {
    pub product_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}
