# Issue #2 — Fix status filter whitespace bug

**Goal**: make `GET /tasks?status=todo` match tasks whose stored status contains trailing or leading whitespace. Currently the filter compares strings directly, so `" todo"` (with a leading space) silently fails to match the seed task with status `"todo"`.

> This issue requires adding a `?status=` filter first, then exposing the bug, then fixing it. Two of the three steps are the agent's job. The filter and the failing test come from this issue.

## Step 1 — Add a failing test

In `tests/integration.rs`, add a test that:

1. Creates a task with `status: "todo"` (clean).
2. Mutates its stored status to `" todo"` (leading whitespace — write directly to the in-memory store, since there's no PATCH endpoint yet).
3. Calls `GET /tasks?status=todo`.
4. Expects the response to include the task.

This test should fail against the current implementation because the filter compares strings strictly.

## Step 2 — Add the `?status=` filter

In `src/handlers.rs`, parse an optional `status` query parameter and filter the returned list.

## Step 3 — Fix the bug

Trim whitespace from both sides of the comparison. Either:

- Trim each stored task's status when comparing, or
- Trim the query parameter once and compare against stored values directly.

Pick the approach that minimises per-request work. (Hint: trim once, on the query side.)

## Acceptance criteria

- `GET /tasks?status=todo` returns the task whose stored status is `" todo"` (with whitespace).
- `GET /tasks?status=todo` does not return tasks with other statuses.
- `GET /tasks` (no query parameter) returns everything (unchanged behaviour).
- A regression test covers the whitespace case.

## Branch

`feat/issue-2-fix-status-filter-bug`

## Suggested commit shape

- `test: add failing test for status filter whitespace`
- `feat: add ?status= query filter on GET /tasks`
- `fix: trim whitespace in status filter comparison`

## Verification

- `cargo test` — all tests pass.
- The whitespace regression test passes specifically because of the fix.