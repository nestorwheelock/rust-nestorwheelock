use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub parent_id: Option<i64>,
    pub template: Option<String>,
    pub show_in_nav: bool,
    pub display_order: i32,
    pub show_posts_from_category_id: Option<i64>,
    pub show_posts_with_tag_id: Option<i64>,
    pub posts_per_page: i32,
    pub is_published: bool,
    pub visibility: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Page {
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let page = sqlx::query_as::<_, Page>(
            "SELECT id, title, slug, body, parent_id, template, show_in_nav, display_order,
                    show_posts_from_category_id, show_posts_with_tag_id, posts_per_page,
                    is_published, visibility, author_id, created_at, updated_at
             FROM posts_page WHERE slug = $1 AND is_published = true"
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;
        Ok(page)
    }

    pub async fn list_nav_pages(pool: &PgPool) -> Result<Vec<Self>> {
        let pages = sqlx::query_as::<_, Page>(
            "SELECT id, title, slug, body, parent_id, template, show_in_nav, display_order,
                    show_posts_from_category_id, show_posts_with_tag_id, posts_per_page,
                    is_published, visibility, author_id, created_at, updated_at
             FROM posts_page
             WHERE is_published = true AND show_in_nav = true AND visibility = 'PUBLIC' AND parent_id IS NULL
             ORDER BY display_order, title"
        )
        .fetch_all(pool)
        .await?;
        Ok(pages)
    }
}
