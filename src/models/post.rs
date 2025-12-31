use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;
use crate::privacy::allowed_visibilities;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: Option<String>,
    pub body: String,
    pub location: Option<String>,
    pub author_id: i32,
    pub visibility: String,
    pub is_draft: bool,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub category_id: Option<i64>,
    pub source_platform: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>> {
        let post = sqlx::query_as::<_, Post>(
            "SELECT id, title, body, location, author_id, visibility, is_draft, is_pinned,
                    is_archived, is_deleted, category_id, source_platform, like_count,
                    comment_count, share_count, created_at, updated_at
             FROM posts_post WHERE id = $1 AND is_draft = false AND is_deleted = false"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(post)
    }

    pub async fn list_public(pool: &PgPool, user_tier: Option<&str>, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let visibilities = allowed_visibilities(user_tier);
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, body, location, author_id, visibility, is_draft, is_pinned,
                    is_archived, is_deleted, category_id, source_platform, like_count,
                    comment_count, share_count, created_at, updated_at
             FROM posts_post
             WHERE is_draft = false AND is_deleted = false AND visibility = ANY($1)
             ORDER BY is_pinned DESC, created_at DESC
             LIMIT $2 OFFSET $3"
        )
        .bind(&visibilities)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub async fn list_by_category(pool: &PgPool, category_id: i64, user_tier: Option<&str>, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let visibilities = allowed_visibilities(user_tier);
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, body, location, author_id, visibility, is_draft, is_pinned,
                    is_archived, is_deleted, category_id, source_platform, like_count,
                    comment_count, share_count, created_at, updated_at
             FROM posts_post
             WHERE is_draft = false AND is_deleted = false AND category_id = $1 AND visibility = ANY($2)
             ORDER BY is_pinned DESC, created_at DESC
             LIMIT $3 OFFSET $4"
        )
        .bind(category_id)
        .bind(&visibilities)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub async fn list_by_tag(pool: &PgPool, tag_id: i64, user_tier: Option<&str>, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let visibilities = allowed_visibilities(user_tier);
        let posts = sqlx::query_as::<_, Post>(
            "SELECT p.id, p.title, p.body, p.location, p.author_id, p.visibility, p.is_draft,
                    p.is_pinned, p.is_archived, p.is_deleted, p.category_id, p.source_platform,
                    p.like_count, p.comment_count, p.share_count, p.created_at, p.updated_at
             FROM posts_post p
             JOIN posts_post_tags pt ON p.id = pt.post_id
             WHERE p.is_draft = false AND p.is_deleted = false AND pt.tag_id = $1 AND p.visibility = ANY($2)
             ORDER BY p.is_pinned DESC, p.created_at DESC
             LIMIT $3 OFFSET $4"
        )
        .bind(tag_id)
        .bind(&visibilities)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub async fn search(pool: &PgPool, query: &str, user_tier: Option<&str>, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let visibilities = allowed_visibilities(user_tier);
        let search_pattern = format!("%{}%", query);
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, body, location, author_id, visibility, is_draft, is_pinned,
                    is_archived, is_deleted, category_id, source_platform, like_count,
                    comment_count, share_count, created_at, updated_at
             FROM posts_post
             WHERE is_draft = false AND is_deleted = false AND visibility = ANY($1)
                   AND (title ILIKE $2 OR body ILIKE $2)
             ORDER BY created_at DESC
             LIMIT $3 OFFSET $4"
        )
        .bind(&visibilities)
        .bind(&search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub fn preview(&self) -> String {
        if self.body.len() > 280 {
            format!("{}...", &self.body[..280])
        } else {
            self.body.clone()
        }
    }
}
