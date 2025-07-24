pub mod audiobooks;

use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn create_router() -> Router<PgPool> {
    Router::new()
        .route("/audiobooks", get(audiobooks::list_audiobooks))
        .route("/audiobooks/:id", get(audiobooks::get_audiobook))
}