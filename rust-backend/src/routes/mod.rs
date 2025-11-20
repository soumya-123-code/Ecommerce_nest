use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use crate::controllers::{self, AppState};

/// Create all application routes
/// Django: urls.py → Rust Router
pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check
        .route("/health", get(health_check))

        // Auth routes - Django: accounts/urls.py
        .route("/api/auth/register", post(controllers::register))
        .route("/api/auth/login", post(controllers::login))
        .route("/api/auth/logout", post(controllers::logout))
        .route("/api/auth/profile", get(controllers::get_profile))
        .route("/api/auth/profile", put(controllers::update_profile))
        .route("/api/auth/change-password", post(controllers::change_password))
        .route("/api/auth/password-reset", post(controllers::request_password_reset))
        .route("/api/auth/password-reset/confirm", post(controllers::confirm_password_reset))
        .route("/api/auth/apply-vendor", post(controllers::apply_vendor))

        // Product routes - Django: products/urls.py
        .route("/api/products", get(controllers::list_products))
        .route("/api/products/featured", get(controllers::get_featured_products))
        .route("/api/products/:slug", get(controllers::get_product))
        .route("/api/products/:id/rate", post(controllers::rate_product))
        .route("/api/products/:id/ratings", get(controllers::get_product_ratings))
        .route("/api/categories", get(controllers::get_categories))
        .route("/api/search", get(controllers::search_products))

        // Order routes - Django: orders/urls.py
        .route("/api/orders", get(controllers::list_orders))
        .route("/api/orders", post(controllers::create_order))
        .route("/api/orders/:id", get(controllers::get_order))
        .route("/api/payments/initiate", post(controllers::initiate_payment))
        .route("/api/coupons/apply", post(controllers::apply_coupon))

        // Vendor routes - Django: supplier_panel/urls.py
        .route("/api/vendor/products", get(controllers::list_vendor_products))
        .route("/api/vendor/products", post(controllers::create_vendor_product))
        .route("/api/vendor/products/:id", put(controllers::update_vendor_product))
        .route("/api/vendor/orders", get(controllers::list_vendor_orders))
        .route("/api/vendor/orders/:id", put(controllers::update_vendor_order))
        .route("/api/vendor/wallet", get(controllers::get_vendor_wallet))
        .route("/api/vendor/payments/request", post(controllers::request_vendor_payout))

        // Blog routes - Django: blog/urls.py
        .route("/api/blog/posts", get(controllers::list_posts))
        .route("/api/blog/posts/:slug", get(controllers::get_post))
        .route("/api/blog/posts/:id/comments", get(controllers::get_post_comments))
        .route("/api/blog/posts/:id/comments", post(controllers::create_comment))
        .route("/api/blog/posts/:id/view", post(controllers::record_post_view))

        // Home/Settings routes - Django: home/urls.py, settings/urls.py
        .route("/api/home", get(controllers::get_home_page))
        .route("/api/carousels", get(controllers::get_carousels))
        .route("/api/settings", get(controllers::get_site_settings))
        .route("/api/pages/:slug", get(controllers::get_page))

        // Contact routes - Django: contact/urls.py
        .route("/api/newsletter/subscribe", post(controllers::subscribe_newsletter))
        .route("/api/newsletter/unsubscribe", post(controllers::unsubscribe_newsletter))
        .route("/api/contact", post(controllers::submit_contact))

        // Webhook routes - Django: orders/views.py (webhooks)
        .route("/api/webhooks/stripe", post(controllers::stripe_webhook))
        .route("/api/webhooks/razorpay", post(controllers::razorpay_webhook))
        .route("/api/webhooks/paypal", post(controllers::paypal_webhook))

        // Misc routes
        .route("/api/countries", get(controllers::get_countries))

        // Add middleware layers
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
