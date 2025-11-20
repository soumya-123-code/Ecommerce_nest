use serde::{Deserialize, Serialize};

// ==================== Common DTOs ====================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn success_message(data: T, message: &str) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: Some(message.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            success: true,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl PaginationInfo {
    pub fn new(page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).min(100).max(1)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
}

// ==================== Blog DTOs ====================

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub author: AuthorInfo,
    pub views_count: i32,
    pub comments_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub author: String,
    pub views_count: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AuthorInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub user: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

// ==================== Contact DTOs ====================

#[derive(Debug, Deserialize, validator::Validate)]
pub struct ContactRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub subject: String,
    #[validate(length(min = 10))]
    pub message: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct NewsletterRequest {
    #[validate(email)]
    pub email: String,
}

// ==================== Home/Settings DTOs ====================

#[derive(Debug, Serialize)]
pub struct HomePageResponse {
    pub carousels: Vec<CarouselResponse>,
    pub featured_products: Vec<super::ProductListResponse>,
    pub categories: Vec<super::CategoryTreeResponse>,
    pub ads: HomeAdsResponse,
}

#[derive(Debug, Serialize)]
pub struct CarouselResponse {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub image: String,
    pub link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HomeAdsResponse {
    pub sidebar: Vec<AdResponse>,
    pub middle: Vec<AdResponse>,
    pub daily_deals: Vec<AdResponse>,
}

#[derive(Debug, Serialize)]
pub struct AdResponse {
    pub id: i32,
    pub title: Option<String>,
    pub image: String,
    pub link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SiteSettingsResponse {
    pub site_name: String,
    pub site_logo: Option<String>,
    pub favicon: Option<String>,
    pub meta_description: Option<String>,
    pub footer_text: Option<String>,
}
