# Issue #3 — Add `?page=` and `?limit=` pagination to `GET /tasks`

**Goal**: paginate the list endpoint so callers don't have to receive the full list.

## Acceptance criteria

- `GET /tasks?limit=10` returns at most 10 tasks.
- `GET /tasks?page=2&limit=10` returns tasks 11–20 (1-indexed pages).
- Default `page=1`, default `limit=50` (or pick a sensible default — document your choice).
- Out-of-range pages return an empty array, not an error.
- The integration test suite covers: default page, second page, last partial page, page beyond the end.

## Design notes

- Pagination is over the existing list order — keep insertion order, don't sort yet (a separate issue can add stable sorting if needed).
- Document the page/limit convention in the `CreateTask`-style struct in `handlers.rs`.
- Reject `page=0` and `limit=0` with `400 Bad Request` — explicit error is better than silently treating as 1.

## Branch

`feat/issue-3-add-pagination`

## Files to touch

- `src/handlers.rs` — extract `Query<Pagination>` from the request; return the slice.
- `src/store.rs` — add a `paginated(page, limit) -> Vec<&Task>` helper (or compute it inline; either is fine).
- `tests/integration.rs` — at least three new tests covering the cases above.

## Suggested commit shape

- `feat: add ?page= and ?limit= pagination to GET /tasks`
- `test: cover default, second, partial, and out-of-range pages`
- `fix: reject page=0 and limit=0 with 400 Bad Request` (if you find you need to)

## Verification

- `cargo test` — all tests pass.
- `cargo fmt --check && cargo clippy --all-targets -- -D warnings` — clean.