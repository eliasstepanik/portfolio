use sqlx::{migrate, postgres::PgPoolOptions, PgPool};
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

pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    info!("Running database migrations");
    
    migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            let db_error = sqlx::Error::Protocol(format!("Migration error: {}", e));
            AppError::Database(db_error)
        })?;
    
    info!("Database migrations completed successfully");
    
    Ok(())
}
