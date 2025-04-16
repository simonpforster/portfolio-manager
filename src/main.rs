pub mod observability;
pub mod repository;
mod project_router;

use axum::{middleware, Router};
use std::{env, net::SocketAddr, sync::Arc};
use firestore::FirestoreDb;
use tracing::{info, warn};
use crate::observability::init_tracing;
use crate::observability::propagators::extract_context;
use crate::repository::project_repository::ProjectRepository;

// App state that will be shared across all routes
#[derive(Debug)]
struct AppState {
    project_repository: Arc<ProjectRepository>
}


#[tokio::main]
async fn main() {
    let service_name: String = env::var("K_SERVICE").unwrap_or("oliviazuo-portfolio".into());
    let gcp_project_id: String = env::var("GCP_PROJECT_ID").expect("env var GCP_PROJECT_ID not configured");

    // Initialize tracing for nice logging
    let _ = init_tracing(service_name, gcp_project_id.clone()).await;

    let port: u16 = env::var("PORT").unwrap_or_else(|_| {
        warn!("env var PORT not configured, defaulting to 8080");
        "8080".into()
    }).parse::<u16>().expect("env var PORT must be a valid number");

    // Init DB
    let db: FirestoreDb = FirestoreDb::new(&gcp_project_id).await.expect("Could not initiate DB client");

    let proj_repo = ProjectRepository::new(db);

    let thingy = Arc::new(proj_repo);

    // Create shared application state
    let state = Arc::new(AppState {  project_repository: thingy.clone() });

    // Set up our application with routes
    let app = Router::new()
        .nest("/v1", project_router::router(state))
        .layer(middleware::from_fn(extract_context));

    // Run our application
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on {}", addr);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}