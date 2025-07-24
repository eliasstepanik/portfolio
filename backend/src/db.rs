use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::error::AppError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    info!("Creating database connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    info!("Database connection pool created successfully");
    
    Ok(pool)
}