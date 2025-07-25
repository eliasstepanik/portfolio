use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;

use crate::{error::AppError, models::Project};

#[derive(Deserialize, Debug)]
pub struct ProjectFilters {
    pub language: Option<String>,
    pub technology: Option<String>,
    pub featured: Option<bool>,
}

pub async fn list_projects(
    State(pool): State<PgPool>,
    Query(filters): Query<ProjectFilters>,
) -> Result<Json<Vec<Project>>, AppError> {
    debug!("Fetching projects with filters: {:?}", filters);

    let mut query = String::from(
        "SELECT id, name, description, github_url, demo_url, technologies, 
         primary_language, stars_count, featured, image_url, created_at, updated_at 
         FROM projects WHERE 1=1",
    );

    if let Some(lang) = filters.language {
        query.push_str(&format!(" AND primary_language = '{lang}'"));
    }

    if let Some(tech) = filters.technology {
        query.push_str(&format!(" AND '{tech}' = ANY(technologies)"));
    }

    if let Some(featured) = filters.featured {
        query.push_str(&format!(" AND featured = {featured}"));
    }

    query.push_str(" ORDER BY featured DESC, stars_count DESC");

    let projects = sqlx::query_as::<_, Project>(&query)
        .fetch_all(&pool)
        .await?;

    debug!("Found {} projects", projects.len());

    Ok(Json(projects))
}

pub async fn get_project(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Project>, AppError> {
    debug!("Fetching project with id: {}", id);

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, description, github_url, demo_url, technologies, 
         primary_language, stars_count, featured, image_url, created_at, updated_at 
         FROM projects WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    debug!("Found project: {}", project.name);

    Ok(Json(project))
}
