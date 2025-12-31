use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub id: i64,
    pub user_id: i32,
    pub tier: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub full_name: Option<String>,
    pub nickname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Profile {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i32) -> Result<Option<Self>> {
        let profile = sqlx::query_as::<_, Profile>(
            "SELECT id, user_id, tier, bio, location, full_name, nickname, created_at, updated_at
             FROM accounts_profile WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        Ok(profile)
    }

    pub fn is_admin(&self) -> bool {
        self.tier == "ADMIN"
    }

    pub fn is_friend(&self) -> bool {
        matches!(self.tier.as_str(), "FRIEND" | "CLOSE_FRIEND" | "ADMIN")
    }

    pub fn is_close_friend(&self) -> bool {
        matches!(self.tier.as_str(), "CLOSE_FRIEND" | "ADMIN")
    }

    pub fn display_name(&self) -> String {
        if let Some(ref nickname) = self.nickname {
            if !nickname.is_empty() {
                return nickname.clone();
            }
        }
        if let Some(ref full_name) = self.full_name {
            if !full_name.is_empty() {
                return full_name.clone();
            }
        }
        format!("User {}", self.user_id)
    }
}
