# Issue #1 — Add `DELETE /tasks/:id`

**Goal**: add the missing endpoint that closes the basic CRUD surface.

## Acceptance criteria

- New endpoint `DELETE /tasks/:id`.
- Returns `204 No Content` when the task exists and is deleted.
- Returns `404 Not Found` when the task does not exist.
- Existing seed data is unchanged.
- An integration test covers both the success and the not-found case.

## Branch

`feat/issue-1-add-delete-endpoint`

## Files to touch

- `src/handlers.rs` — add `delete_task` handler.
- `src/store.rs` — add a `remove(&mut self, id: &Uuid) -> bool` method.
- `src/bin/taskforge.rs` — wire up `.delete(handlers::delete_task)` on the `tasks/:id` route.
- `tests/integration.rs` — add `delete_task_removes_it` and `delete_missing_task_returns_404`.

## Suggested commit shape (matches Conventional Commits)

- `feat: add DELETE /tasks/:id endpoint`
- `test: cover DELETE success and not-found paths`

## Expected diff snapshot

```rust
// src/handlers.rs — addition
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
```

```rust
// src/store.rs — addition
pub fn remove(&mut self, id: &Uuid) -> bool {
    self.tasks.remove(id).is_some()
}
```

```rust
// src/bin/taskforge.rs — change
.route(
    "/tasks/:id",
    get(handlers::get_task).delete(handlers::delete_task),
)
```

## Verification

- `cargo test` — all tests pass, including two new ones.
- `cargo fmt --check && cargo clippy --all-targets -- -D warnings` — clean.