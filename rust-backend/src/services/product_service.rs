use diesel::prelude::*;
use rust_decimal::Decimal;
use slug::slugify;

use crate::db::DbConnection;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

pub struct ProductService;

impl ProductService {
    /// Get all products with optional filtering
    /// Django: products/views.py - product_list
    pub fn get_products(
        conn: &mut DbConnection,
        category_slug: Option<&str>,
        min_price: Option<Decimal>,
        max_price: Option<Decimal>,
        search: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<Product>, i64)> {
        let mut query = products::table
            .filter(products::is_active.eq(true))
            .into_boxed();

        if let Some(search_term) = search {
            let pattern = format!("%{}%", search_term);
            query = query.filter(
                products::name.like(&pattern)
                    .or(products::description.like(&pattern))
            );
        }

        if let Some(min) = min_price {
            query = query.filter(products::price.ge(min));
        }

        if let Some(max) = max_price {
            query = query.filter(products::price.le(max));
        }

        // Get total count
        let total = query.clone().count().get_result::<i64>(conn)?;

        // Get paginated results
        let products = query
            .order(products::created_at.desc())
            .offset((page - 1) * per_page)
            .limit(per_page)
            .load::<Product>(conn)?;

        Ok((products, total))
    }

    /// Get product by slug
    /// Django: products/views.py - product_detail
    pub fn get_product_by_slug(conn: &mut DbConnection, slug: &str) -> AppResult<Product> {
        let product = products::table
            .filter(products::slug.eq(slug))
            .filter(products::is_active.eq(true))
            .first::<Product>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        // Increment view count
        diesel::update(products::table.find(product.id))
            .set(products::views_count.eq(products::views_count + 1))
            .execute(conn)?;

        Ok(product)
    }

    /// Get product images
    pub fn get_product_images(conn: &mut DbConnection, product_id: i32) -> AppResult<Vec<ProductImage>> {
        product_images::table
            .filter(product_images::product_id.eq(product_id))
            .order(product_images::sort_order.asc())
            .load::<ProductImage>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get product sizes
    pub fn get_product_sizes(conn: &mut DbConnection, product_id: i32) -> AppResult<Vec<ProductSize>> {
        product_sizes::table
            .filter(product_sizes::product_id.eq(product_id))
            .load::<ProductSize>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get product ratings with distribution
    /// Django: products/views.py - product_ratings_view
    pub fn get_product_ratings(
        conn: &mut DbConnection,
        product_id: i32,
    ) -> AppResult<(Vec<ProductRating>, f64, [i32; 5])> {
        let ratings = product_ratings::table
            .filter(product_ratings::product_id.eq(product_id))
            .order(product_ratings::created_at.desc())
            .load::<ProductRating>(conn)?;

        let total = ratings.len() as f64;
        let average = if total > 0.0 {
            ratings.iter().map(|r| r.rating as f64).sum::<f64>() / total
        } else {
            0.0
        };

        // Calculate distribution
        let mut distribution = [0i32; 5];
        for rating in &ratings {
            if rating.rating >= 1 && rating.rating <= 5 {
                distribution[(rating.rating - 1) as usize] += 1;
            }
        }

        Ok((ratings, average, distribution))
    }

    /// Create product rating
    pub fn create_rating(
        conn: &mut DbConnection,
        product_id: i32,
        user_id: i32,
        rating: i32,
        comment: Option<String>,
    ) -> AppResult<ProductRating> {
        // Check if user already rated
        let existing = product_ratings::table
            .filter(product_ratings::product_id.eq(product_id))
            .filter(product_ratings::user_id.eq(user_id))
            .first::<ProductRating>(conn)
            .optional()?;

        if existing.is_some() {
            return Err(AppError::Validation("You have already rated this product".to_string()));
        }

        let new_rating = NewProductRating {
            product_id,
            user_id,
            rating,
            comment,
        };

        diesel::insert_into(product_ratings::table)
            .values(&new_rating)
            .execute(conn)?;

        product_ratings::table
            .order(product_ratings::id.desc())
            .first::<ProductRating>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Create product (vendor)
    /// Django: supplier_panel/views.py - supplier_add_product
    pub fn create_product(
        conn: &mut DbConnection,
        vendor_id: i32,
        name: &str,
        mini_category_id: i32,
        description: &str,
        price: Decimal,
        discount_price: Option<Decimal>,
        stock: i32,
        sku: Option<String>,
    ) -> AppResult<Product> {
        let slug = generate_unique_slug(conn, name)?;

        let new_product = NewProduct {
            vendor_id,
            mini_category_id,
            name: name.to_string(),
            slug,
            description: description.to_string(),
            price,
            discount_price,
            stock,
            sku,
            image: None,
        };

        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)?;

        products::table
            .order(products::id.desc())
            .first::<Product>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Update product (vendor)
    pub fn update_product(
        conn: &mut DbConnection,
        product_id: i32,
        vendor_id: i32,
        updates: UpdateProduct,
    ) -> AppResult<Product> {
        // Verify ownership
        let product = products::table
            .find(product_id)
            .first::<Product>(conn)?;

        if product.vendor_id != vendor_id {
            return Err(AppError::Forbidden("Not your product".to_string()));
        }

        diesel::update(products::table.find(product_id))
            .set(&updates)
            .execute(conn)?;

        products::table
            .find(product_id)
            .first::<Product>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get category tree
    pub fn get_category_tree(conn: &mut DbConnection) -> AppResult<Vec<SuperCategory>> {
        super_categories::table
            .filter(super_categories::is_active.eq(true))
            .load::<SuperCategory>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get featured products
    pub fn get_featured_products(conn: &mut DbConnection, limit: i64) -> AppResult<Vec<Product>> {
        products::table
            .filter(products::is_active.eq(true))
            .filter(products::is_featured.eq(true))
            .order(products::created_at.desc())
            .limit(limit)
            .load::<Product>(conn)
            .map_err(|e| AppError::Database(e))
    }
}

fn generate_unique_slug(conn: &mut DbConnection, name: &str) -> AppResult<String> {
    let base_slug = slugify(name);
    let mut slug = base_slug.clone();
    let mut counter = 1;

    loop {
        let exists = products::table
            .filter(products::slug.eq(&slug))
            .count()
            .get_result::<i64>(conn)? > 0;

        if !exists {
            break;
        }

        slug = format!("{}-{}", base_slug, counter);
        counter += 1;
    }

    Ok(slug)
}
