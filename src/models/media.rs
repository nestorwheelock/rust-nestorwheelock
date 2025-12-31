use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaLibrary {
    pub id: i64,
    pub file: String,
    pub content_hash: String,
    pub media_type: String,
    pub original_filename: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub mime_type: Option<String>,
    pub uploaded_by_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostMedia {
    pub id: i64,
    pub post_id: i64,
    pub library_item_id: i64,
    pub order: i32,
    pub custom_alt_text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostMediaWithItem {
    pub id: i64,
    pub post_id: i64,
    #[sqlx(rename = "order")]
    pub display_order: i32,
    pub custom_alt_text: String,
    pub file: String,
    pub media_type: String,
    pub original_filename: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

impl MediaLibrary {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>> {
        let media = sqlx::query_as::<_, MediaLibrary>(
            "SELECT id, file, content_hash, media_type, original_filename, file_size,
                    width, height, mime_type, uploaded_by_id, created_at
             FROM posts_medialibrary WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(media)
    }

    pub fn url(&self) -> String {
        format!("/media/{}", self.file)
    }
}

impl PostMedia {
    pub async fn list_for_post(pool: &PgPool, post_id: i64) -> Result<Vec<PostMediaWithItem>> {
        let media = sqlx::query_as::<_, PostMediaWithItem>(
            r#"SELECT pm.id, pm.post_id, pm.order, pm.custom_alt_text,
                      ml.file, ml.media_type, ml.original_filename, ml.width, ml.height
               FROM posts_postmedia pm
               JOIN posts_medialibrary ml ON pm.library_item_id = ml.id
               WHERE pm.post_id = $1
               ORDER BY pm.order, pm.created_at"#
        )
        .bind(post_id)
        .fetch_all(pool)
        .await?;
        Ok(media)
    }

    pub async fn get_featured_for_post(pool: &PgPool, post_id: i64) -> Result<Option<PostMediaWithItem>> {
        let media = sqlx::query_as::<_, PostMediaWithItem>(
            r#"SELECT pm.id, pm.post_id, pm.order, pm.custom_alt_text,
                      ml.file, ml.media_type, ml.original_filename, ml.width, ml.height
               FROM posts_postmedia pm
               JOIN posts_medialibrary ml ON pm.library_item_id = ml.id
               WHERE pm.post_id = $1
               ORDER BY pm.order
               LIMIT 1"#
        )
        .bind(post_id)
        .fetch_optional(pool)
        .await?;
        Ok(media)
    }
}

impl PostMediaWithItem {
    pub fn url(&self) -> String {
        format!("/media/{}", self.file)
    }
}
