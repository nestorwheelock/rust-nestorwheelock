use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>> {
        let category = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, parent_id, display_order, is_active, created_at, updated_at
             FROM posts_category WHERE slug = $1 AND is_active = true"
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;
        Ok(category)
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>> {
        let category = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, parent_id, display_order, is_active, created_at, updated_at
             FROM posts_category WHERE id = $1 AND is_active = true"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(category)
    }

    pub async fn list_root(pool: &PgPool) -> Result<Vec<Self>> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, parent_id, display_order, is_active, created_at, updated_at
             FROM posts_category WHERE parent_id IS NULL AND is_active = true
             ORDER BY display_order, name"
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT id, name, slug, description, parent_id, display_order, is_active, created_at, updated_at
             FROM posts_category WHERE is_active = true
             ORDER BY display_order, name"
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }
}
