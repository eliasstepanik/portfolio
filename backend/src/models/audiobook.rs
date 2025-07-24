use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AudioBook {
    pub id: i32,  // PostgreSQL SERIAL type maps to i32
    pub title: String,
    pub author: String,
    pub duration: String,
    pub audio_url: String,
    pub cover_url: String,
    pub description: String,
}