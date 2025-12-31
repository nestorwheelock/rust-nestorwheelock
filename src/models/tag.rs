use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT id, name, slug, is_active, description, created_at
             FROM posts_tag WHERE slug = $1 AND is_active = true"
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;
        Ok(tag)
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT id, name, slug, is_active, description, created_at
             FROM posts_tag WHERE id = $1 AND is_active = true"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(tag)
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT id, name, slug, is_active, description, created_at
             FROM posts_tag WHERE is_active = true
             ORDER BY name"
        )
        .fetch_all(pool)
        .await?;
        Ok(tags)
    }

    pub async fn list_for_post(pool: &PgPool, post_id: i64) -> Result<Vec<Self>> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT t.id, t.name, t.slug, t.is_active, t.description, t.created_at
             FROM posts_tag t
             JOIN posts_post_tags pt ON t.id = pt.tag_id
             WHERE pt.post_id = $1 AND t.is_active = true
             ORDER BY t.name"
        )
        .bind(post_id)
        .fetch_all(pool)
        .await?;
        Ok(tags)
    }
}
