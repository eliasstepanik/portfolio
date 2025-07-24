use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use tracing::{debug, error};

use crate::{error::AppError, models::AudioBook};

pub async fn list_audiobooks(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<AudioBook>>, AppError> {
    debug!("Fetching all audiobooks");
    
    let audiobooks = sqlx::query_as!(
        AudioBook,
        "SELECT id, title, author, duration, audio_url, cover_url, description 
         FROM audiobooks 
         ORDER BY id"
    )
    .fetch_all(&pool)
    .await?;
    
    debug!("Found {} audiobooks", audiobooks.len());
    
    Ok(Json(audiobooks))
}

pub async fn get_audiobook(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<AudioBook>, AppError> {
    debug!("Fetching audiobook with id: {}", id);
    
    let audiobook = sqlx::query_as!(
        AudioBook,
        "SELECT id, title, author, duration, audio_url, cover_url, description 
         FROM audiobooks 
         WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;
    
    debug!("Found audiobook: {}", audiobook.title);
    
    Ok(Json(audiobook))
}