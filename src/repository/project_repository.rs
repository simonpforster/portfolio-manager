use std::collections::HashMap;
use std::sync::RwLock;
use firestore::{path, FirestoreDb, FirestoreQueryDirection, FirestoreResult};
use futures_core::stream::BoxStream;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug)]
pub struct ProjectRepository {
    db: FirestoreDb,
}

const PROJECT_COLLECTION_NAME: &str = "projects";

impl ProjectRepository {
    pub fn new(db: FirestoreDb) -> Self {
        ProjectRepository {
            db
        }
    }

    pub async fn get_projects_by_owner(&self, owner: &str) -> Vec<Project> {
        let stream: BoxStream<FirestoreResult<Project>> = self.db.fluent().select()
            .from(PROJECT_COLLECTION_NAME)
            .filter(|q| {
                q.field("owner").eq(owner)
            })
            .order_by([(
                path!(Project::year),
                FirestoreQueryDirection::Descending,
            )])
            .obj()
            .stream_query_with_errors()
            .await.unwrap();
        let projects: Vec<Project> = stream.try_collect().await.unwrap();
        info!("Projects fetched for owner: {}", owner);
        projects
    }

    pub async fn get_project_by_document_id(&self, document_id: &str) -> Option<Project> {
        self.db.fluent().select()
            .by_id_in(PROJECT_COLLECTION_NAME)
            .obj()
            .one(document_id)
            .await.unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    #[serde(rename = "_firestore_id")]
    document_id: String,

    #[serde(rename = "projectName")]
    pub(crate) project_name: String,
    #[serde(rename = "projectId")]
    pub(crate) project_id: String,
    owner: String,

    pub(crate) year: u16,
    #[serde(rename = "type")]
    project_type: String,
    pub(crate) references: Option<HashMap<String, String>>,
    pub(crate) description: Option<String>,
    pub(crate) tags: Vec<String>,
}