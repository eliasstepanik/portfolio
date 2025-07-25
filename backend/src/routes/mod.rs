use axum::{routing::get, Router};

mod projects;

pub fn create_router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/projects", get(projects::list_projects))
        .route("/projects/:id", get(projects::get_project))
}
