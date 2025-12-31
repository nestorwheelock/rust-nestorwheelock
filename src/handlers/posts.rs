use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Path, State};

use crate::db::DbPool;
use crate::error::{AppError, Result};
use crate::models::{Category, Page, Post, PostMedia, Profile, Tag};
use crate::models::media::PostMediaWithItem;

#[derive(Template)]
#[template(path = "detail.html")]
pub struct DetailTemplate {
    pub post: Post,
    pub media: Vec<PostMediaWithItem>,
    pub tags: Vec<Tag>,
    pub category: Option<Category>,
    pub nav_pages: Vec<Page>,
    pub current_path: String,
    pub show_dates: bool,
    pub user: Option<Profile>,
}

pub async fn detail(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    let post = Post::find_by_id(&pool, id).await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let media = PostMedia::list_for_post(&pool, post.id).await?;
    let tags = Tag::list_for_post(&pool, post.id).await?;
    let category = if let Some(cat_id) = post.category_id {
        Category::find_by_id(&pool, cat_id).await?
    } else {
        None
    };
    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(DetailTemplate {
        post,
        media,
        tags,
        category,
        nav_pages,
        current_path: format!("/posts/{}/", id),
        show_dates: false,
        user: None,
    })
}
