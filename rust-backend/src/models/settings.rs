use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{site_settings, carousels, home_ads};

// ==================== Site Settings Model ====================
// Django: settings.SiteSetting → Rust: SiteSetting struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = site_settings)]
pub struct SiteSetting {
    pub id: i32,
    pub site_name: String,
    pub site_logo: Option<String>,
    pub favicon: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub footer_text: Option<String>,
    pub maintenance_mode: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = site_settings)]
pub struct UpdateSiteSetting {
    pub site_name: Option<String>,
    pub site_logo: Option<String>,
    pub favicon: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub footer_text: Option<String>,
    pub maintenance_mode: Option<bool>,
}

// ==================== Carousel Model ====================
// Django: home.Carousel → Rust: Carousel struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = carousels)]
pub struct Carousel {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub image: String,
    pub link: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = carousels)]
pub struct NewCarousel {
    pub title: String,
    pub subtitle: Option<String>,
    pub image: String,
    pub link: Option<String>,
    pub sort_order: i32,
}

// ==================== Home Ad Model ====================
// Django: home.HomeAdSidebar/HomeAdMiddlebar/etc → Rust: HomeAd struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = home_ads)]
pub struct HomeAd {
    pub id: i32,
    pub ad_type: String,
    pub title: Option<String>,
    pub image: String,
    pub link: Option<String>,
    pub position: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = home_ads)]
pub struct NewHomeAd {
    pub ad_type: String,
    pub title: Option<String>,
    pub image: String,
    pub link: Option<String>,
    pub position: String,
    pub sort_order: i32,
    pub starts_at: Option<NaiveDateTime>,
    pub ends_at: Option<NaiveDateTime>,
}
