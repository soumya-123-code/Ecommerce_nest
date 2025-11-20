use axum::{
    extract::{State, Path, Query, Json},
};

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::services::ProductService;
use crate::utils::errors::AppResult;

/// GET /api/products
/// Django: products/views.py - product_list
pub async fn list_products(
    State(state): State<std::sync::Arc<AppState>>,
    Query(params): Query<ProductFilterParams>,
) -> AppResult<Json<PaginatedResponse<ProductListResponse>>> {
    let mut conn = state.pool.get()?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let (products, total) = ProductService::get_products(
        &mut conn,
        params.category.as_deref(),
        params.min_price,
        params.max_price,
        params.search.as_deref(),
        page,
        per_page,
    )?;

    let items: Vec<ProductListResponse> = products
        .into_iter()
        .map(|p| ProductListResponse {
            id: p.id,
            name: p.name,
            slug: p.slug,
            price: p.price.to_string(),
            discount_price: p.discount_price.map(|d| d.to_string()),
            image: p.image,
            rating: 0.0, // Would need to calculate from ratings
            reviews_count: 0,
            is_featured: p.is_featured,
        })
        .collect();

    Ok(Json(PaginatedResponse {
        data: items,
        pagination: PaginationInfo::new(page, per_page, total),
    }))
}

/// GET /api/products/:slug
/// Django: products/views.py - product_detail
pub async fn get_product(
    State(state): State<std::sync::Arc<AppState>>,
    Path(slug): Path<String>,
) -> AppResult<Json<ApiResponse<ProductResponse>>> {
    let mut conn = state.pool.get()?;

    let product = ProductService::get_product_by_slug(&mut conn, &slug)?;
    let images = ProductService::get_product_images(&mut conn, product.id)?;
    let sizes = ProductService::get_product_sizes(&mut conn, product.id)?;
    let (_, avg_rating, distribution) = ProductService::get_product_ratings(&mut conn, product.id)?;

    let response = ProductResponse {
        id: product.id,
        name: product.name,
        slug: product.slug,
        description: product.description,
        price: product.price.to_string(),
        discount_price: product.discount_price.map(|d| d.to_string()),
        stock: product.stock,
        sku: product.sku,
        image: product.image,
        images: images
            .into_iter()
            .map(|i| ProductImageResponse {
                id: i.id,
                image: i.image,
                alt_text: i.alt_text,
            })
            .collect(),
        sizes: sizes
            .into_iter()
            .map(|s| ProductSizeResponse {
                id: s.id,
                size: s.size,
                stock: s.stock,
                price_adjustment: s.price_adjustment.to_string(),
            })
            .collect(),
        category: CategoryBreadcrumb {
            super_category: CategoryInfo { id: 0, name: String::new(), slug: String::new() },
            main_category: CategoryInfo { id: 0, name: String::new(), slug: String::new() },
            sub_category: CategoryInfo { id: 0, name: String::new(), slug: String::new() },
            mini_category: CategoryInfo { id: product.mini_category_id, name: String::new(), slug: String::new() },
        },
        vendor: VendorInfo {
            id: product.vendor_id,
            name: String::new(), // Would need to join with profiles
        },
        rating: RatingInfo {
            average: avg_rating,
            count: distribution.iter().sum(),
            distribution: RatingDistribution {
                five_star: distribution[4],
                four_star: distribution[3],
                three_star: distribution[2],
                two_star: distribution[1],
                one_star: distribution[0],
            },
        },
        is_active: product.is_active,
        is_featured: product.is_featured,
        views_count: product.views_count,
        created_at: product.created_at.to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/products/:id/rate
/// Django: products/views.py - rate_product_view
pub async fn rate_product(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(product_id): Path<i32>,
    Json(payload): Json<CreateRatingRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    ProductService::create_rating(
        &mut conn,
        product_id,
        auth_user.user_id,
        payload.rating,
        payload.comment,
    )?;

    Ok(Json(MessageResponse::new("Rating submitted successfully")))
}

/// GET /api/products/:id/ratings
pub async fn get_product_ratings(
    State(state): State<std::sync::Arc<AppState>>,
    Path(product_id): Path<i32>,
) -> AppResult<Json<ApiResponse<RatingInfo>>> {
    let mut conn = state.pool.get()?;

    let (_, avg, distribution) = ProductService::get_product_ratings(&mut conn, product_id)?;

    Ok(Json(ApiResponse::success(RatingInfo {
        average: avg,
        count: distribution.iter().sum(),
        distribution: RatingDistribution {
            five_star: distribution[4],
            four_star: distribution[3],
            three_star: distribution[2],
            two_star: distribution[1],
            one_star: distribution[0],
        },
    })))
}

/// GET /api/categories
pub async fn get_categories(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<CategoryTreeResponse>>>> {
    let mut conn = state.pool.get()?;

    let categories = ProductService::get_category_tree(&mut conn)?;

    let response: Vec<CategoryTreeResponse> = categories
        .into_iter()
        .map(|c| CategoryTreeResponse {
            id: c.id,
            name: c.name,
            slug: c.slug,
            image: c.image,
            children: vec![], // Would need to load nested categories
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/products/featured
pub async fn get_featured_products(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<ProductListResponse>>>> {
    let mut conn = state.pool.get()?;

    let products = ProductService::get_featured_products(&mut conn, 12)?;

    let items: Vec<ProductListResponse> = products
        .into_iter()
        .map(|p| ProductListResponse {
            id: p.id,
            name: p.name,
            slug: p.slug,
            price: p.price.to_string(),
            discount_price: p.discount_price.map(|d| d.to_string()),
            image: p.image,
            rating: 0.0,
            reviews_count: 0,
            is_featured: p.is_featured,
        })
        .collect();

    Ok(Json(ApiResponse::success(items)))
}
