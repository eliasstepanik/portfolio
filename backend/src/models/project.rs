use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub github_url: String,
    pub demo_url: Option<String>,
    pub technologies: Vec<String>,
    pub primary_language: String,
    pub stars_count: i32,
    pub featured: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
