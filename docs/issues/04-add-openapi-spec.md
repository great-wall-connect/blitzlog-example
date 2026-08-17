# Issue #4 — Generate OpenAPI spec at `GET /openapi.json`

**Goal**: serve a machine-readable OpenAPI 3.1 document describing the current API surface.

This is the largest of the demo issues — it touches dependencies, multiple files, and document generation. It exercises Blitzlog's ability to plan and execute non-trivial work.

## Approach options

Pick **one** of the following. The recommended path is A because it keeps dependencies minimal and the spec data lives next to the code.

### A. Hand-written OpenAPI document (recommended)

- Add a new module `src/openapi.rs` that builds an `serde_json::Value` representing the spec.
- Expose `GET /openapi.json` that returns it as JSON.
- Cover every endpoint currently in the binary: `/health`, `/tasks` (GET + POST), `/tasks/:id` (GET), and any future ones added by issues #1 and #3.

### B. Procedural via `utoipa`

- Add `utoipa = { version = "5", features = ["axum_extras"] }` and `utoipa-swagger-ui` (optional).
- Annotate each handler with `#[utoipa::path(...)]`.
- Generate the spec at startup from a `#[derive(OpenApi)]` struct.
- Trade-off: more dependencies, more build time, less control over output formatting.

## Acceptance criteria

- `GET /openapi.json` returns `200 OK` with `Content-Type: application/json`.
- The document is valid OpenAPI 3.1 (validate with [`swagger-cli`](https://www.npmjs.com/package/swagger-cli) or similar in CI if you want belt-and-braces).
- All current endpoints are documented with request/response schemas.
- An integration test fetches `/openapi.json` and asserts it parses as JSON and contains the expected `paths` keys.

## Branch

`feat/issue-4-add-openapi-spec`

## Files to touch

- `Cargo.toml` — add dependencies if you chose path B; nothing new for path A.
- `src/openapi.rs` (new) — the spec builder.
- `src/lib.rs` — `pub mod openapi;`.
- `src/bin/taskforge.rs` — wire up `.route("/openapi.json", get(handlers::openapi_spec))`.
- `src/handlers.rs` — add `openapi_spec` handler.
- `tests/integration.rs` — integration test for `/openapi.json`.

## Suggested commit shape (path A)

- `feat: hand-write OpenAPI 3.1 document at /openapi.json`
- `test: integration test for /openapi.json`

## Verification

- `cargo test` — all tests pass.
- `cargo fmt --check && cargo clippy --all-targets -- -D warnings` — clean.
- (Optional) `npx swagger-cli validate http://localhost:3000/openapi.json` — passes.