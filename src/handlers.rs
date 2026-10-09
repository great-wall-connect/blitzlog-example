use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{models::Task, store::TaskStore};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn list_tasks(State(store): State<Arc<RwLock<TaskStore>>>) -> Json<Vec<Task>> {
    let store = store.read().await;
    Json(store.all().into_iter().cloned().collect())
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub status: Option<String>,
}

pub async fn create_task(
    State(store): State<Arc<RwLock<TaskStore>>>,
    Json(input): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let task = Task {
        id: Uuid::new_v4(),
        title: input.title,
        status: input.status.unwrap_or_else(|| "todo".to_string()),
        created_at: chrono::Utc::now(),
    };
    let mut store = store.write().await;
    store.insert(task.clone());
    (StatusCode::CREATED, Json(task))
}

pub async fn get_task(
    State(store): State<Arc<RwLock<TaskStore>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, StatusCode> {
    let store = store.read().await;
    store
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn delete_task(
    State(store): State<Arc<RwLock<TaskStore>>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut store = store.write().await;
    if store.remove(&id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
