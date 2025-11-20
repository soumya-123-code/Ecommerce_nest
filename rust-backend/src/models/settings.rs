use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{site_settings, carousels, home_ads, site_social_links, contact_info, support_numbers, home_page_themes, pages};

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

// ==================== Site Social Links Model ====================
// Django: settings.SocailLinks → Rust: SiteSocialLink struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = site_social_links)]
pub struct SiteSocialLink {
    pub id: i32,
    pub platform: String,
    pub url: String,
    pub icon: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = site_social_links)]
pub struct NewSiteSocialLink {
    pub platform: String,
    pub url: String,
    pub icon: Option<String>,
}

// ==================== Contact Info Model ====================
// Django: settings.ContactInfo → Rust: ContactInfo struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = contact_info)]
pub struct ContactInfo {
    pub id: i32,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub working_hours: Option<String>,
    pub map_embed: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = contact_info)]
pub struct UpdateContactInfo {
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub working_hours: Option<String>,
    pub map_embed: Option<String>,
}

// ==================== Support Number Model ====================
// Django: settings.SupportNumber → Rust: SupportNumber struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = support_numbers)]
pub struct SupportNumber {
    pub id: i32,
    pub label: String,
    pub number: String,
    pub is_whatsapp: bool,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = support_numbers)]
pub struct NewSupportNumber {
    pub label: String,
    pub number: String,
    pub is_whatsapp: bool,
}

// ==================== Home Page Theme Model ====================
// Django: settings.HomePageTheme → Rust: HomePageTheme struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = home_page_themes)]
pub struct HomePageTheme {
    pub id: i32,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
    pub font_family: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = home_page_themes)]
pub struct UpdateHomePageTheme {
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
    pub font_family: Option<String>,
    pub is_active: Option<bool>,
}

// ==================== Pages List Model ====================
// Django: pages.PagesList → Rust: Page struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = pages)]
pub struct NewPage {
    pub title: String,
    pub slug: String,
    pub content: String,
}
