# Repository Guidelines

## Project Structure & Module Organization

ai-cli-apps is a single binary crate; all CLI wiring, tool metadata, and async workflows live in `src/main.rs`. Dependencies live in `Cargo.toml` (edition 2024). Use the `Makefile` for repeatable builds, `README.md` for CLI examples, and `TODO.md` for roadmap items. Build outputs belong in `target/`; never commit that directory or downloaded binaries.

## Build, Test, and Development Commands

- `make build` – `cargo build --release` into `target/release/ai-cli-apps`.
- `make install` – rebuilds then installs into your Cargo bin for smoke tests.
- `make test` – executes the Rust test suite; run before every PR.
- `cargo run -- list` / `cargo run -- install claude` – exercise CLI flows without global installs.
- `cargo fmt && cargo clippy --all-targets --all-features -D warnings` – enforce formatting and lint cleanliness.

## Coding Style & Naming Conventions

Rustfmt (4-space indentation, trailing commas in multiline lists) is the source of truth. Keep modules, functions, and local bindings in `snake_case`, types in `PascalCase`, and constants in `SCREAMING_SNAKE_CASE`. Favor small helpers that return `anyhow::Result` with `.context(...)` for actionable errors. For concurrency, reuse the `tokio::spawn` + `futures::future::join_all` pattern from the version checks and keep user-facing strings near the CLI definitions.

## Testing Guidelines

Place unit tests beside the code under `#[cfg(test)] mod tests` and name cases with behavior-focused verbs (e.g., `it_marks_missing_tool`). Use `cargo test -- --nocapture` when printing diagnostics, and lean on `#[tokio::test]` for async helpers. Avoid hitting live APIs in tests: mock the HTTP layer or gate network checks behind feature flags so CI stays deterministic.

## Commit & Pull Request Guidelines

Recent history shows concise, imperative subjects such as `claude install & uninstall`; continue that format and keep summaries under ~60 characters. Each PR should describe the motivation, list the user-visible behavior changes, and paste key command output (`make test`, `cargo fmt`, or sample `ai-cli-apps list`). Link the relevant issue or TODO entry, call out any new dependencies, and attach screenshots or terminal transcripts when modifying interactive flows.

## Security & Configuration Tips

Install/uninstall routines write into `$HOME/.local` and `/usr/local/bin`; re-check permissions before scripting changes. Network calls hit the npm and GitHub APIs via `reqwest`, so respect proxy settings and keep tokens out of logs. Never hardcode user-specific paths or IDs—derive them from `std::env` and validate before deleting files.
