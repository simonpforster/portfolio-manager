use crate::repository::project_repository::Project;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tracing::info;

pub(crate) fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/projects/{document_id}", get(get_handler))
        .route("/owners/{owner_id}/projects", get(get_projects_for_owner_handler)) // get all projects for an owner
        .with_state(state)
}

async fn get_handler(Path(document_id): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let project: Project = state.project_repository.get_project_by_document_id(&document_id).await.unwrap();
    info!("Project fetched: {:?}", project);
    axum::response::Json(project)
}

async fn get_projects_for_owner_handler(Path(owner_id): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let projects: Vec<Project> = state.project_repository.get_projects_by_owner(&owner_id).await;
    info!("Projects fetched for owner: {:?}", projects);
    axum::response::Json(projects)
}