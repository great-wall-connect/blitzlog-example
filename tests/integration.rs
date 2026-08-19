use std::net::SocketAddr;

use serde_json::{json, Value};
use taskforge::{handlers, store::TaskStore};

async fn spawn_app() -> SocketAddr {
    let store = std::sync::Arc::new(tokio::sync::RwLock::new(TaskStore::with_seed()));
    let app = axum::Router::new()
        .route("/health", axum::routing::get(handlers::health))
        .route(
            "/tasks",
            axum::routing::get(handlers::list_tasks).post(handlers::create_task),
        )
        .route(
            "/tasks/:id",
            axum::routing::get(handlers::get_task).delete(handlers::delete_task),
        )
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    addr
}

#[tokio::test]
async fn health_works() {
    let addr = spawn_app().await;
    let url = format!("http://{}/health", addr);
    let resp = reqwest::get(url).await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn list_tasks_returns_seeded() {
    let addr = spawn_app().await;
    let url = format!("http://{}/tasks", addr);
    let resp = reqwest::get(url).await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
    let body: Vec<Value> = resp.json().await.unwrap();
    assert_eq!(body.len(), 3);
}

#[tokio::test]
async fn create_task_round_trips() {
    let addr = spawn_app().await;
    let url = format!("http://{}/tasks", addr);
    let resp = reqwest::Client::new()
        .post(url)
        .json(&json!({ "title": "Write integration test" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::CREATED);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["title"], "Write integration test");
    assert_eq!(body["status"], "todo");
}

#[tokio::test]
async fn get_missing_task_returns_404() {
    let addr = spawn_app().await;
    let url = format!("http://{}/tasks/00000000-0000-0000-0000-000000000000", addr);
    let resp = reqwest::get(url).await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_task_removes_it() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let list_url = format!("http://{}/tasks", addr);
    let resp = client.get(&list_url).send().await.unwrap();
    let body: Vec<Value> = resp.json().await.unwrap();
    assert_eq!(body.len(), 3);
    let id = body[0]["id"].as_str().unwrap().to_string();

    let delete_url = format!("http://{}/tasks/{}", addr, id);
    let resp = client.delete(&delete_url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NO_CONTENT);

    let resp = client.get(&delete_url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);

    let resp = client.get(&list_url).send().await.unwrap();
    let body: Vec<Value> = resp.json().await.unwrap();
    assert_eq!(body.len(), 2);
}

#[tokio::test]
async fn delete_missing_task_returns_404() {
    let addr = spawn_app().await;
    let url = format!("http://{}/tasks/00000000-0000-0000-0000-000000000000", addr);
    let resp = reqwest::Client::new().delete(&url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
}
