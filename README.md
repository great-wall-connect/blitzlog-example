# blitzlog-example — taskforge

> Worked example for **[blitzlog](https://github.com/great-wall-connect/blitzlog)**.

A small Rust HTTP service called **`taskforge`** that demonstrates the Blitzlog autonomous coding pipeline end-to-end. Four seed issues in `docs/issues/` exercise the full agent loop — each is sized so one autonomous run should close it.

## What is this?

`taskforge` is a deliberately small task-tracker API. It has:

- `GET /health` — health check.
- `GET /tasks` — list tasks (seeded with three on startup).
- `POST /tasks` — create a task.
- `GET /tasks/:id` — fetch one task.

The four demo issues under `docs/issues/` walk through the kind of work Blitzlog is built to handle:

1. **[`#1 Add DELETE /tasks/:id`](docs/issues/01-add-delete-endpoint.md)** — happy path: add a new endpoint.
2. **[`#2 Fix status filter whitespace bug`](docs/issues/02-fix-status-filter-bug.md)** — fix from a failing test.
3. **[`#3 Add ?page=&limit pagination`](docs/issues/03-add-pagination.md)** — multi-file feature.
4. **[`#4 Generate OpenAPI at /openapi.json`](docs/issues/04-add-openapi-spec.md)** — non-trivial dependency work.

To watch Blitzlog close any of these:

1. Push this repo to GitHub.
2. Deploy [Blitzlog](https://github.com/great-wall-connect/blitzlog) against it (or a fork).
3. Add the [`autonomous`](https://github.com/great-wall-connect/blitzlog#4-label-an-issue) label to one of the issues.
4. Blitzlog clones the repo on a fresh EC2 spot instance, runs the agent, opens a PR, and shuts itself down.

## Stack

- Rust 1.96+ (stable channel pinned via `rust-toolchain.toml`).
- Axum 0.7 + Tokio + Serde + UUID + Chrono.
- In-memory store seeded on startup (replace with a real DB in your own service).
- `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` as the test/lint loop.

## Layout

```
blitzlog-example/
├── AGENTS.md               # conventions the agent + contributors follow
├── Cargo.toml              # taskforge crate
├── Cargo.lock              # committed (binary)
├── mise.toml               # toolchain + tasks (test, lint, build)
├── rust-toolchain.toml     # stable channel pinned
├── src/
│   ├── lib.rs              # re-exports modules
│   ├── handlers.rs         # axum handlers
│   ├── models.rs           # Task struct
│   ├── store.rs            # in-memory TaskStore (seeded with three tasks)
│   └── bin/
│       └── taskforge.rs    # binary entrypoint
├── tests/
│   └── integration.rs      # 4 integration tests against the running app
├── docs/
│   └── issues/             # 4 demo issues for Blitzlog to close
└── .github/
    └── workflows/ci.yml    # cargo build / test / fmt / clippy
```

## Local development

```bash
# Install tools
mise install

# Run the binary
cargo run --bin taskforge

# Hit it
curl localhost:3000/health
curl localhost:3000/tasks

# Run tests
cargo test

# Lint
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Licence

[MIT](LICENSE) © 2026 Great Wall Connect Limited.

Maintained by Great Wall Connect Limited — `admin@greatwallconnect.com`.