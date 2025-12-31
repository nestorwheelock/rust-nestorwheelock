use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Query, State};
use serde::Deserialize;

use crate::db::DbPool;
use crate::error::Result;
use crate::models::{Page, Post, PostMedia, Profile, Tag};
use crate::handlers::feed::PostWithMedia;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub page: Option<i64>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub posts: Vec<PostWithMedia>,
    pub query: String,
    pub nav_pages: Vec<Page>,
    pub current_path: String,
    pub current_tag: Option<Tag>,
    pub show_dates: bool,
    pub has_next_page: bool,
    pub next_page: i64,
    pub user: Option<Profile>,
}

pub async fn search_page(
    State(pool): State<DbPool>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse> {
    let query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let per_page = 10i64;
    let offset = (page - 1) * per_page;

    let posts = if !query.is_empty() {
        Post::search(&pool, &query, None, per_page + 1, offset).await?
    } else {
        vec![]
    };

    let has_next = posts.len() as i64 > per_page;
    let posts: Vec<Post> = posts.into_iter().take(per_page as usize).collect();

    let mut posts_with_media = Vec::new();
    for post in posts {
        let featured_media = PostMedia::get_featured_for_post(&pool, post.id).await?;
        let tags = Tag::list_for_post(&pool, post.id).await?;
        posts_with_media.push(PostWithMedia {
            id: post.id,
            title: post.title,
            body: post.body,
            location: post.location,
            source_platform: post.source_platform,
            created_at: post.created_at,
            featured_media,
            tags,
        });
    }

    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(SearchTemplate {
        posts: posts_with_media,
        query,
        nav_pages,
        current_path: "/search/".to_string(),
        current_tag: None,
        show_dates: false,
        has_next_page: has_next,
        next_page: page + 1,
        user: None,
    })
}
