use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use crate::schema::{posts, comments, post_views, post_reports, newsletters, contact_messages};

// ==================== Post Model ====================
// Django: blog.Post → Rust: Post struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub is_published: bool,
    pub views_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub is_published: Option<bool>,
}

// ==================== Comment Model ====================
// Django: blog.Comment → Rust: Comment struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
    pub is_approved: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
}

// ==================== Post View Model ====================
// Django: blog.PostView → Rust: PostView struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = post_views)]
pub struct PostView {
    pub id: i32,
    pub post_id: i32,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub is_mobile: bool,
    pub viewed_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = post_views)]
pub struct NewPostView {
    pub post_id: i32,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub is_mobile: bool,
}

// ==================== Newsletter Model ====================
// Django: contact.Newsletter → Rust: Newsletter struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = newsletters)]
pub struct Newsletter {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
    pub subscribed_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = newsletters)]
pub struct NewNewsletter {
    pub email: String,
}

// ==================== Contact Message Model ====================
// Django: contact.MessagesList → Rust: ContactMessage struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = contact_messages)]
pub struct ContactMessage {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = contact_messages)]
pub struct NewContactMessage {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

// ==================== Post Report Model ====================
// Django: reports.PostReport → Rust: PostReport struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = post_reports)]
pub struct PostReport {
    pub id: i32,
    pub post_id: i32,
    pub date: NaiveDate,
    pub impressions: i32,
    pub unique_visitors: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = post_reports)]
pub struct NewPostReport {
    pub post_id: i32,
    pub date: NaiveDate,
    pub impressions: i32,
    pub unique_visitors: i32,
}
