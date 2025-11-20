use axum::{
    extract::{State, Path, Query, Json},
};
use diesel::prelude::*;

use crate::controllers::AppState;
use crate::dto::*;
use crate::middlewares::auth::{AuthUser, OptionalAuthUser};
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

/// GET /api/blog/posts
pub async fn list_posts(
    State(state): State<std::sync::Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<PostListResponse>>> {
    let mut conn = state.pool.get()?;

    let page = params.page();
    let per_page = params.per_page();

    let total = posts::table
        .filter(posts::is_published.eq(true))
        .count()
        .get_result::<i64>(&mut conn)?;

    let post_list = posts::table
        .inner_join(users::table)
        .filter(posts::is_published.eq(true))
        .order(posts::created_at.desc())
        .offset(params.offset())
        .limit(per_page)
        .select((posts::all_columns, users::first_name, users::last_name))
        .load::<(Post, String, String)>(&mut conn)?;

    let items: Vec<PostListResponse> = post_list
        .into_iter()
        .map(|(post, first_name, last_name)| PostListResponse {
            id: post.id,
            title: post.title,
            slug: post.slug,
            excerpt: post.excerpt,
            featured_image: post.featured_image,
            author: format!("{} {}", first_name, last_name),
            views_count: post.views_count,
            created_at: post.created_at.to_string(),
        })
        .collect();

    Ok(Json(PaginatedResponse {
        data: items,
        pagination: PaginationInfo::new(page, per_page, total),
    }))
}

/// GET /api/blog/posts/:slug
pub async fn get_post(
    State(state): State<std::sync::Arc<AppState>>,
    Path(slug): Path<String>,
) -> AppResult<Json<ApiResponse<PostResponse>>> {
    let mut conn = state.pool.get()?;

    let (post, first_name, last_name) = posts::table
        .inner_join(users::table)
        .filter(posts::slug.eq(&slug))
        .filter(posts::is_published.eq(true))
        .select((posts::all_columns, users::first_name, users::last_name))
        .first::<(Post, String, String)>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // Increment view count
    diesel::update(posts::table.find(post.id))
        .set(posts::views_count.eq(posts::views_count + 1))
        .execute(&mut conn)?;

    // Get comments count
    let comments_count = comments::table
        .filter(comments::post_id.eq(post.id))
        .filter(comments::is_approved.eq(true))
        .count()
        .get_result::<i64>(&mut conn)? as i32;

    Ok(Json(ApiResponse::success(PostResponse {
        id: post.id,
        title: post.title,
        slug: post.slug,
        content: post.content,
        excerpt: post.excerpt,
        featured_image: post.featured_image,
        author: AuthorInfo {
            id: post.author_id,
            name: format!("{} {}", first_name, last_name),
        },
        views_count: post.views_count,
        comments_count,
        created_at: post.created_at.to_string(),
    })))
}

/// GET /api/blog/posts/:id/comments
pub async fn get_post_comments(
    State(state): State<std::sync::Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<CommentResponse>>>> {
    let mut conn = state.pool.get()?;

    let comment_list = comments::table
        .inner_join(users::table)
        .filter(comments::post_id.eq(post_id))
        .filter(comments::is_approved.eq(true))
        .order(comments::created_at.desc())
        .select((comments::all_columns, users::username))
        .load::<(Comment, String)>(&mut conn)?;

    let items: Vec<CommentResponse> = comment_list
        .into_iter()
        .map(|(comment, username)| CommentResponse {
            id: comment.id,
            user: username,
            content: comment.content,
            created_at: comment.created_at.to_string(),
        })
        .collect();

    Ok(Json(ApiResponse::success(items)))
}

/// POST /api/blog/posts/:id/comments
pub async fn create_comment(
    State(state): State<std::sync::Arc<AppState>>,
    auth_user: AuthUser,
    Path(post_id): Path<i32>,
    Json(payload): Json<CreateCommentRequest>,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    // Verify post exists
    posts::table
        .find(post_id)
        .first::<Post>(&mut conn)
        .map_err(|_| AppError::NotFound("Post not found".to_string()))?;

    let new_comment = NewComment {
        post_id,
        user_id: auth_user.user_id,
        content: payload.content,
    };

    diesel::insert_into(comments::table)
        .values(&new_comment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("Comment submitted for approval")))
}

/// POST /api/blog/posts/:id/view
pub async fn record_post_view(
    State(state): State<std::sync::Arc<AppState>>,
    Path(post_id): Path<i32>,
    headers: axum::http::HeaderMap,
) -> AppResult<Json<MessageResponse>> {
    let mut conn = state.pool.get()?;

    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let new_view = NewPostView {
        post_id,
        ip_address,
        user_agent,
        browser: None,
        os: None,
        device: None,
        is_mobile: false,
    };

    diesel::insert_into(post_views::table)
        .values(&new_view)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse::new("View recorded")))
}
