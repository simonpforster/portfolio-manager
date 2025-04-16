use crate::repository::project_repository::Project;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub(crate) fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/projects/{document_id}", get(get_handler))
        .route("/owners/{owner_id}/projects", get(get_handler)) // get all projects for an owner
        .route("/owners/{owner_id}/projects/{document_id}", get(get_handler)) // get a specific project by project id from an owner
        .with_state(state)
}

async fn get_handler(Path((document_id)): Path<(String)>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let project: Project = state.project_repository.get_project_by_document_id(&document_id).await.unwrap();
    axum::response::Json(project)
}