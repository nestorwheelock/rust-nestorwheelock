use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Path, Query, State};
use serde::Deserialize;

use crate::db::DbPool;
use crate::error::Result;
use crate::models::{Category, Page, Post, PostMedia, Profile, Tag};
use crate::models::media::PostMediaWithItem;

#[derive(Deserialize)]
pub struct FeedParams {
    pub page: Option<i64>,
    pub tag: Option<String>,
}

#[derive(Clone)]
pub struct PostWithMedia {
    pub id: i64,
    pub title: Option<String>,
    pub body: String,
    pub location: Option<String>,
    pub source_platform: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub featured_media: Option<PostMediaWithItem>,
    pub tags: Vec<Tag>,
}

impl PostWithMedia {
    pub fn preview(&self) -> String {
        if self.body.len() > 280 {
            format!("{}...", &self.body[..280])
        } else {
            self.body.clone()
        }
    }
}

#[derive(Template)]
#[template(path = "feed.html")]
pub struct FeedTemplate {
    pub posts: Vec<PostWithMedia>,
    pub nav_pages: Vec<Page>,
    pub current_tag: Option<Tag>,
    pub current_path: String,
    pub show_dates: bool,
    pub has_next_page: bool,
    pub next_page: i64,
    pub user: Option<Profile>,
}

#[derive(Template)]
#[template(path = "partials/feed_items.html")]
pub struct FeedItemsTemplate {
    pub posts: Vec<PostWithMedia>,
    pub current_tag: Option<Tag>,
    pub show_dates: bool,
    pub has_next_page: bool,
    pub next_page: i64,
}

pub async fn homepage(
    State(pool): State<DbPool>,
    Query(params): Query<FeedParams>,
) -> Result<impl IntoResponse> {
    let page = params.page.unwrap_or(1);
    let per_page = 10i64;
    let offset = (page - 1) * per_page;

    let posts = Post::list_public(&pool, None, per_page + 1, offset).await?;
    let has_next = posts.len() as i64 > per_page;
    let posts: Vec<Post> = posts.into_iter().take(per_page as usize).collect();

    let posts_with_media = enrich_posts(&pool, posts).await?;
    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(FeedTemplate {
        posts: posts_with_media,
        nav_pages,
        current_tag: None,
        current_path: "/".to_string(),
        show_dates: false, // Anonymous users don't see dates
        has_next_page: has_next,
        next_page: page + 1,
        user: None,
    })
}

pub async fn feed_partial(
    State(pool): State<DbPool>,
    Query(params): Query<FeedParams>,
) -> Result<impl IntoResponse> {
    let page = params.page.unwrap_or(1);
    let per_page = 10i64;
    let offset = (page - 1) * per_page;

    let current_tag = if let Some(ref slug) = params.tag {
        Tag::find_by_slug(&pool, slug).await?
    } else {
        None
    };

    let posts = if let Some(ref tag) = current_tag {
        Post::list_by_tag(&pool, tag.id, None, per_page + 1, offset).await?
    } else {
        Post::list_public(&pool, None, per_page + 1, offset).await?
    };

    let has_next = posts.len() as i64 > per_page;
    let posts: Vec<Post> = posts.into_iter().take(per_page as usize).collect();
    let posts_with_media = enrich_posts(&pool, posts).await?;

    Ok(FeedItemsTemplate {
        posts: posts_with_media,
        current_tag,
        show_dates: false,
        has_next_page: has_next,
        next_page: page + 1,
    })
}

pub async fn browse(
    State(pool): State<DbPool>,
    Query(params): Query<FeedParams>,
) -> Result<impl IntoResponse> {
    homepage(State(pool), Query(params)).await
}

pub async fn category_feed(
    State(pool): State<DbPool>,
    Path(slug): Path<String>,
    Query(params): Query<FeedParams>,
) -> Result<impl IntoResponse> {
    let category = Category::find_by_slug(&pool, &slug).await?;
    let page = params.page.unwrap_or(1);
    let per_page = 10i64;
    let offset = (page - 1) * per_page;

    let posts = if let Some(ref cat) = category {
        Post::list_by_category(&pool, cat.id, None, per_page + 1, offset).await?
    } else {
        vec![]
    };

    let has_next = posts.len() as i64 > per_page;
    let posts: Vec<Post> = posts.into_iter().take(per_page as usize).collect();
    let posts_with_media = enrich_posts(&pool, posts).await?;
    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(FeedTemplate {
        posts: posts_with_media,
        nav_pages,
        current_tag: None,
        current_path: format!("/category/{}/", slug),
        show_dates: false,
        has_next_page: has_next,
        next_page: page + 1,
        user: None,
    })
}

pub async fn tag_feed(
    State(pool): State<DbPool>,
    Path(slug): Path<String>,
    Query(params): Query<FeedParams>,
) -> Result<impl IntoResponse> {
    let tag = Tag::find_by_slug(&pool, &slug).await?;
    let page = params.page.unwrap_or(1);
    let per_page = 10i64;
    let offset = (page - 1) * per_page;

    let posts = if let Some(ref t) = tag {
        Post::list_by_tag(&pool, t.id, None, per_page + 1, offset).await?
    } else {
        vec![]
    };

    let has_next = posts.len() as i64 > per_page;
    let posts: Vec<Post> = posts.into_iter().take(per_page as usize).collect();
    let posts_with_media = enrich_posts(&pool, posts).await?;
    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(FeedTemplate {
        posts: posts_with_media,
        nav_pages,
        current_tag: tag,
        current_path: format!("/tags/{}/", slug),
        show_dates: false,
        has_next_page: has_next,
        next_page: page + 1,
        user: None,
    })
}

async fn enrich_posts(pool: &DbPool, posts: Vec<Post>) -> Result<Vec<PostWithMedia>> {
    let mut result = Vec::new();
    for post in posts {
        let featured_media = PostMedia::get_featured_for_post(pool, post.id).await?;
        let tags = Tag::list_for_post(pool, post.id).await?;
        result.push(PostWithMedia {
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
    Ok(result)
}
