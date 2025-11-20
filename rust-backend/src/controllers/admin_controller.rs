use axum::{
    extract::{State, Path, Query, Json},
};
use diesel::prelude::*;

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::AuthUser;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

// ==================== BANK ACCOUNTS ====================

/// GET /api/bank-accounts
pub async fn list_bank_accounts(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<BankAccount>>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    let accounts = bank_accounts::table
        .filter(bank_accounts::profile_id.eq(profile.id))
        .load::<BankAccount>(&mut conn)?;

    Ok(Json(ApiResponse::success(accounts)))
}

/// POST /api/bank-accounts
pub async fn create_bank_account(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<NewBankAccount>,
) -> AppResult<Json<ApiResponse<BankAccount>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    let mut new_account = payload;
    new_account.profile_id = profile.id;

    diesel::insert_into(bank_accounts::table)
        .values(&new_account)
        .execute(&mut conn)?;

    let account = bank_accounts::table
        .order(bank_accounts::id.desc())
        .first::<BankAccount>(&mut conn)?;

    Ok(Json(ApiResponse::success(account)))
}

/// DELETE /api/bank-accounts/:id
pub async fn delete_bank_account(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    diesel::delete(
        bank_accounts::table
            .filter(bank_accounts::id.eq(id))
            .filter(bank_accounts::profile_id.eq(profile.id))
    ).execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Bank account deleted")))
}

// ==================== SOCIAL LINKS ====================

/// GET /api/social-links
pub async fn list_social_links(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<SocialLink>>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    let links = social_links::table
        .filter(social_links::profile_id.eq(profile.id))
        .load::<SocialLink>(&mut conn)?;

    Ok(Json(ApiResponse::success(links)))
}

/// POST /api/social-links
pub async fn create_social_link(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<NewSocialLink>,
) -> AppResult<Json<ApiResponse<SocialLink>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    let mut new_link = payload;
    new_link.profile_id = profile.id;

    diesel::insert_into(social_links::table)
        .values(&new_link)
        .execute(&mut conn)?;

    let link = social_links::table
        .order(social_links::id.desc())
        .first::<SocialLink>(&mut conn)?;

    Ok(Json(ApiResponse::success(link)))
}

/// DELETE /api/social-links/:id
pub async fn delete_social_link(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    diesel::delete(
        social_links::table
            .filter(social_links::id.eq(id))
            .filter(social_links::profile_id.eq(profile.id))
    ).execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Social link deleted")))
}

// ==================== CATEGORIES ====================

/// GET /api/categories/super
pub async fn list_super_categories(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<SuperCategory>>>> {
    let mut conn = state.pool.get()?;

    let cats = super_categories::table
        .filter(super_categories::is_active.eq(true))
        .load::<SuperCategory>(&mut conn)?;

    Ok(Json(ApiResponse::success(cats)))
}

/// GET /api/categories/main/:super_id
pub async fn list_main_categories(
    State(state): State<std::sync::Arc<AppState>>,
    Path(super_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<MainCategory>>>> {
    let mut conn = state.pool.get()?;

    let cats = main_categories::table
        .filter(main_categories::super_category_id.eq(super_id))
        .filter(main_categories::is_active.eq(true))
        .load::<MainCategory>(&mut conn)?;

    Ok(Json(ApiResponse::success(cats)))
}

/// GET /api/categories/sub/:main_id
pub async fn list_sub_categories(
    State(state): State<std::sync::Arc<AppState>>,
    Path(main_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<SubCategory>>>> {
    let mut conn = state.pool.get()?;

    let cats = sub_categories::table
        .filter(sub_categories::main_category_id.eq(main_id))
        .filter(sub_categories::is_active.eq(true))
        .load::<SubCategory>(&mut conn)?;

    Ok(Json(ApiResponse::success(cats)))
}

/// GET /api/categories/mini/:sub_id
pub async fn list_mini_categories(
    State(state): State<std::sync::Arc<AppState>>,
    Path(sub_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<MiniCategory>>>> {
    let mut conn = state.pool.get()?;

    let cats = mini_categories::table
        .filter(mini_categories::sub_category_id.eq(sub_id))
        .filter(mini_categories::is_active.eq(true))
        .load::<MiniCategory>(&mut conn)?;

    Ok(Json(ApiResponse::success(cats)))
}

// ==================== PRODUCT IMAGES & SIZES ====================

/// GET /api/products/:id/images
pub async fn list_product_images(
    State(state): State<std::sync::Arc<AppState>>,
    Path(product_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<ProductImage>>>> {
    let mut conn = state.pool.get()?;

    let images = product_images::table
        .filter(product_images::product_id.eq(product_id))
        .order(product_images::sort_order.asc())
        .load::<ProductImage>(&mut conn)?;

    Ok(Json(ApiResponse::success(images)))
}

/// GET /api/products/:id/sizes
pub async fn list_product_sizes(
    State(state): State<std::sync::Arc<AppState>>,
    Path(product_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<ProductSize>>>> {
    let mut conn = state.pool.get()?;

    let sizes = product_sizes::table
        .filter(product_sizes::product_id.eq(product_id))
        .load::<ProductSize>(&mut conn)?;

    Ok(Json(ApiResponse::success(sizes)))
}

// ==================== ORDER DETAILS ====================

/// GET /api/orders/:id/details
pub async fn list_order_details(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<OrderDetail>>>> {
    let mut conn = state.pool.get()?;

    // Verify order belongs to user
    let _order = orders::table
        .filter(orders::id.eq(order_id))
        .filter(orders::user_id.eq(auth_user.user_id))
        .first::<Order>(&mut conn)
        .map_err(|_| AppError::NotFound("Order not found".to_string()))?;

    let details = order_details::table
        .filter(order_details::order_id.eq(order_id))
        .load::<OrderDetail>(&mut conn)?;

    Ok(Json(ApiResponse::success(details)))
}

/// GET /api/orders/:id/suppliers
pub async fn list_order_suppliers(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<OrderSupplier>>>> {
    let mut conn = state.pool.get()?;

    // Verify order belongs to user
    let _order = orders::table
        .filter(orders::id.eq(order_id))
        .filter(orders::user_id.eq(auth_user.user_id))
        .first::<Order>(&mut conn)
        .map_err(|_| AppError::NotFound("Order not found".to_string()))?;

    let suppliers = order_suppliers::table
        .filter(order_suppliers::order_id.eq(order_id))
        .load::<OrderSupplier>(&mut conn)?;

    Ok(Json(ApiResponse::success(suppliers)))
}

// ==================== COUPONS ====================

/// GET /api/coupons
pub async fn list_coupons(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<Coupon>>>> {
    let mut conn = state.pool.get()?;

    let coupon_list = coupons::table
        .filter(coupons::is_active.eq(true))
        .load::<Coupon>(&mut conn)?;

    Ok(Json(ApiResponse::success(coupon_list)))
}

/// GET /api/coupons/:code
pub async fn get_coupon(
    State(state): State<std::sync::Arc<AppState>>,
    Path(code): Path<String>,
) -> AppResult<Json<ApiResponse<Coupon>>> {
    let mut conn = state.pool.get()?;

    let coupon = coupons::table
        .filter(coupons::code.eq(&code))
        .filter(coupons::is_active.eq(true))
        .first::<Coupon>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Coupon not found".to_string()))?;

    Ok(Json(ApiResponse::success(coupon)))
}

// ==================== PAYMENTS ====================

/// GET /api/orders/:id/payment
pub async fn get_order_payment(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Payment>>> {
    let mut conn = state.pool.get()?;

    // Verify order belongs to user
    let _order = orders::table
        .filter(orders::id.eq(order_id))
        .filter(orders::user_id.eq(auth_user.user_id))
        .first::<Order>(&mut conn)
        .map_err(|_| AppError::NotFound("Order not found".to_string()))?;

    let payment = payments::table
        .filter(payments::order_id.eq(order_id))
        .first::<Payment>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;

    Ok(Json(ApiResponse::success(payment)))
}

// ==================== VENDOR PAYMENTS ====================

/// GET /api/vendor/payments
pub async fn list_vendor_payments(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<VendorPayment>>>> {
    let mut conn = state.pool.get()?;

    let profile = profiles::table
        .filter(profiles::user_id.eq(auth_user.user_id))
        .first::<Profile>(&mut conn)?;

    if !profile.is_vendor {
        return Err(AppError::Forbidden("Vendor access required".to_string()));
    }

    let payments_list = vendor_payments::table
        .filter(vendor_payments::vendor_id.eq(profile.id))
        .order(vendor_payments::created_at.desc())
        .load::<VendorPayment>(&mut conn)?;

    Ok(Json(ApiResponse::success(payments_list)))
}

// ==================== SITE SETTINGS ====================

/// GET /api/contact-info
pub async fn get_contact_info(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<ContactInfo>>> {
    let mut conn = state.pool.get()?;

    let info = contact_info::table
        .first::<ContactInfo>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Contact info not found".to_string()))?;

    Ok(Json(ApiResponse::success(info)))
}

/// GET /api/support-numbers
pub async fn list_support_numbers(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<SupportNumber>>>> {
    let mut conn = state.pool.get()?;

    let numbers = support_numbers::table
        .filter(support_numbers::is_active.eq(true))
        .load::<SupportNumber>(&mut conn)?;

    Ok(Json(ApiResponse::success(numbers)))
}

/// GET /api/site-social-links
pub async fn list_site_social_links(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<SiteSocialLink>>>> {
    let mut conn = state.pool.get()?;

    let links = site_social_links::table
        .filter(site_social_links::is_active.eq(true))
        .load::<SiteSocialLink>(&mut conn)?;

    Ok(Json(ApiResponse::success(links)))
}

/// GET /api/home-theme
pub async fn get_home_theme(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<HomePageTheme>>> {
    let mut conn = state.pool.get()?;

    let theme = home_page_themes::table
        .filter(home_page_themes::is_active.eq(true))
        .first::<HomePageTheme>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Theme not found".to_string()))?;

    Ok(Json(ApiResponse::success(theme)))
}

// ==================== ADS ====================

/// GET /api/ads/sidebar
pub async fn list_sidebar_ads(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<HomeAd>>>> {
    let mut conn = state.pool.get()?;

    let ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("sidebar"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    Ok(Json(ApiResponse::success(ads)))
}

/// GET /api/ads/middle
pub async fn list_middle_ads(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<HomeAd>>>> {
    let mut conn = state.pool.get()?;

    let ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("middle"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    Ok(Json(ApiResponse::success(ads)))
}

/// GET /api/ads/daily
pub async fn list_daily_ads(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<HomeAd>>>> {
    let mut conn = state.pool.get()?;

    let ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::position.eq("daily"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    Ok(Json(ApiResponse::success(ads)))
}

/// GET /api/ads/hot-deals
pub async fn list_hot_deal_ads(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<HomeAd>>>> {
    let mut conn = state.pool.get()?;

    let ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::ad_type.eq("hot_deal"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    Ok(Json(ApiResponse::success(ads)))
}

/// GET /api/ads/supplier
pub async fn list_supplier_ads(
    State(state): State<std::sync::Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<HomeAd>>>> {
    let mut conn = state.pool.get()?;

    let ads = home_ads::table
        .filter(home_ads::is_active.eq(true))
        .filter(home_ads::ad_type.eq("supplier"))
        .order(home_ads::sort_order.asc())
        .load::<HomeAd>(&mut conn)?;

    Ok(Json(ApiResponse::success(ads)))
}
