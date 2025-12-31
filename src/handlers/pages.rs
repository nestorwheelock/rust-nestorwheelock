use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Path, State};

use crate::db::DbPool;
use crate::error::{AppError, Result};
use crate::models::{Page, Post, Profile};
use crate::handlers::feed::PostWithMedia;

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub page: Page,
    pub posts: Vec<PostWithMedia>,
    pub nav_pages: Vec<Page>,
    pub current_path: String,
    pub user: Option<Profile>,
}

pub async fn page_detail(
    State(pool): State<DbPool>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse> {
    let page = Page::find_by_slug(&pool, &slug).await?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;

    // Check if page pulls posts from category or tag
    let posts = vec![]; // TODO: implement post pulling for pages

    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(PageTemplate {
        page,
        posts,
        nav_pages,
        current_path: format!("/{}/", slug),
        user: None,
    })
}
