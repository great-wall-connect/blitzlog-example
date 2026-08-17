use std::sync::Arc;

use axum::{routing::get, Router};
use taskforge::{handlers, store::TaskStore};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(TaskStore::with_seed()));

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route(
            "/tasks",
            get(handlers::list_tasks).post(handlers::create_task),
        )
        .route("/tasks/:id", get(handlers::get_task))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind 0.0.0.0:3000");

    println!("taskforge listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("axum::serve");
}
