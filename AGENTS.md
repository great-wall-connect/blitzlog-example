# Agent Conventions

This document defines the conventions the autonomous coding agent follows and that contributors match.

## Branch naming

Format: `feat/issue-{N}-{slug}`

Where:
- `N` is the GitHub issue number.
- `slug` is a short lowercase hyphenated description (max 50 chars).

Examples:
- `feat/issue-1-add-delete-endpoint`
- `feat/issue-2-fix-status-filter-bug`

## Commit conventions

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat: add DELETE /tasks/:id endpoint`
- `fix: trim whitespace in status filter`

## Implementation standards

1. **Read the issue** — Fetch full issue body and comments before starting.
2. **Single purpose** — One issue = one feature or fix; no bundled changes.
3. **Tests required** — All new code must have tests (`cargo test`).
4. **No breaking changes** — Without explicit discussion in an issue first.
5. **Preserve coding style** — Match existing code conventions.

## Testing commands

Run before pushing:

| Stack | Command |
|-------|---------|
| Rust  | `cargo test` |
| Lint  | `cargo fmt --check && cargo clippy --all-targets -- -D warnings` |

Use `mise run test` and `mise run lint` to run the same commands via `mise.toml`.

## Pull request

1. Branch from `main`: `git checkout -b feat/issue-{N}-{slug}`.
2. Push: `git push origin HEAD`.
3. Open a PR against `main` via `gh pr create`.
4. Reference the issue: `Closes #N` in the PR body.
5. Wait for CI to pass and a review.
6. **Do not merge** — leave for human review.