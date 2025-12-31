use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContactSubmission {
    pub id: i64,
    pub user_id: Option<i32>,
    pub name: String,
    pub email: String,
    pub oauth_provider: Option<String>,
    pub message: String,
    pub status: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContactSubmission {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl ContactSubmission {
    pub async fn create(
        pool: &PgPool,
        user_id: Option<i32>,
        input: CreateContactSubmission,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Self> {
        let submission = sqlx::query_as::<_, ContactSubmission>(
            r#"INSERT INTO posts_contactsubmission (user_id, name, email, message, status, ip_address, user_agent, created_at, updated_at)
               VALUES ($1, $2, $3, $4, 'NEW', $5, $6, NOW(), NOW())
               RETURNING id, user_id, name, email, oauth_provider, message, status, ip_address, user_agent, created_at, updated_at"#
        )
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.email)
        .bind(&input.message)
        .bind(ip_address)
        .bind(user_agent)
        .fetch_one(pool)
        .await?;
        Ok(submission)
    }
}
