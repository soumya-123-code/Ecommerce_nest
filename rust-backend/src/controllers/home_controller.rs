use axum::{
    extract::{State, Json},
};
use diesel::prelude::*;

use crate::controllers::AppState;
use crate::dto::*;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::AppResult;

/// GET /api/home
pub async fn get_home_page(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<HomePageResponse>>> {
    let mut conn = state.pool.get()?;

    // Get carousels
    let carousel_list = carousels::table
        .filter(carousels::is_active.eq(true))
        .order(carousels::sort_order.asc())
        .load::<Carousel>(&mut conn)?;

    let carousels_response: Vec<CarouselResponse> = carousel_list
        .into_iter()
        .map(|c| CarouselResponse {
            id: c.id,
            title: c.title,
            subtitle: c.subtitle,
            image: c.image,
            link: c.link,
        })
        .collect();

    // Get featured products
    let featured = products::table
        .filter(products::is_active.eq(true))
        .filter(products::is_featured.eq(true))
        .order(products::created_at.desc())
        .limit(12)
        .load::<Product>(&mut conn)?;

    let featured_products: Vec<ProductListResponse> = featured
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

    // Get categories
    let cats = super_categories::table
        .filter(super_categories::is_active.eq(true))
        .load::<SuperCategory>(&mut conn)?;

    let categories: Vec<CategoryTreeResponse> = cats
        .into_iter()
        .map(|c| CategoryTreeResponse {
            id: c.id,
            name: c.name,
            slug: c.slug,
            image: c.image,
            children: vec![],
        })
        .collect();

    // Get ads
    let sidebar_ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("sidebar"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    let middle_ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("middle"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    let daily_ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("daily"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    let map_ad = |ad: HomeAd| AdResponse {
        id: ad.id,
        title: ad.title,
        image: ad.image,
        link: ad.link,
    };

    Ok(Json(ApiResponse::success(HomePageResponse {
        carousels: carousels_response,
        featured_products,
        categories,
        ads: HomeAdsResponse {
            sidebar: sidebar_ads.into_iter().map(map_ad).collect(),
            middle: middle_ads.into_iter().map(map_ad).collect(),
            daily_deals: daily_ads.into_iter().map(map_ad).collect(),
        },
    })))
}

/// GET /api/carousels
pub async fn get_carousels(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<CarouselResponse>>>> {
    let mut conn = state.pool.get()?;

    let carousel_list = carousels::table
        .filter(carousels::is_active.eq(true))
        .order(carousels::sort_order.asc())
        .load::<Carousel>(&mut conn)?;

    let items: Vec<CarouselResponse> = carousel_list
        .into_iter()
        .map(|c| CarouselResponse {
            id: c.id,
            title: c.title,
            subtitle: c.subtitle,
            image: c.image,
            link: c.link,
        })
        .collect();

    Ok(Json(ApiResponse::success(items)))
}

/// GET /api/settings
pub async fn get_site_settings(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<SiteSettingsResponse>>> {
    let mut conn = state.pool.get()?;

    let settings = site_settings::table
        .first::<SiteSetting>(&mut conn)
        .optional()?;

    let response = match settings {
        Some(s) => SiteSettingsResponse {
            site_name: s.site_name,
            site_logo: s.site_logo,
            favicon: s.favicon,
            meta_description: s.meta_description,
            footer_text: s.footer_text,
        },
        None => SiteSettingsResponse {
            site_name: "E-Commerce Store".to_string(),
            site_logo: None,
            favicon: None,
            meta_description: None,
            footer_text: None,
        },
    };

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/pages/:slug
pub async fn get_page(
    State(state): State<std::sync::Arc<AppState>>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> AppResult<Json<ApiResponse<Page>>> {
    let mut conn = state.pool.get()?;

    let page = pages::table
        .filter(pages::slug.eq(&slug))
        .filter(pages::is_published.eq(true))
        .first::<Page>(&mut conn)
        .optional()?
        .ok_or_else(|| crate::utils::errors::AppError::NotFound("Page not found".to_string()))?;

    Ok(Json(ApiResponse::success(page)))
}
