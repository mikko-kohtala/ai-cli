# ai-cli

Manages AI CLI tools and MCP servers.

## Project workflow

Before changing this repository, read and follow
`.agents/skills/project-workflow/SKILL.md` from the repository root.

Repository-specific instructions and explicit user directions take precedence.

## Validation

Validate all work with `make check` (fmt, clippy, tests) before calling it done. `make fmt-fix` applies formatting.

## Adding things

Adding new tool:

1. Create `src/tools/<name>.rs` with `definition()` and `installed_version()`, using the same display name in both
2. In `src/tools/mod.rs`: add `mod <name>;` and the `pub use` re-export, then add to `catalog()` and `installed_versions()`
3. For update checks, add a latest-version source under that same display name in `check_latest_versions()` in `src/versions.rs`

Adding new MCP server:

1. Add function in `src/mcp/servers.rs`, include in `catalog()`

Adding new MCP target:

1. Add function in `src/mcp/targets.rs`, include in `catalog()`

## Code Style

- Return `anyhow::Result` with `.context()` for errors
- `#[tokio::test]` for async tests, mock HTTP in tests
- Concise imperative commit messages
