# ai-cli

Manages AI CLI tools and MCP servers.

## Build Commands

```bash
make check          # check everything: fmt + clippy + test
make build          # release binary
make install        # install binary
make doctor         # run mcp doctor
cargo run -- apps   # list tools
cargo run -- mcp list  # list mcp status
```

## Architecture

### Apps (`src/tools/`)

**`Tool`** - installable CLI tool with `InstallMethod` enum

Adding new tool:

1. Create `src/tools/<name>.rs` with `definition()` and `installed_version()`
2. Add to `catalog()` and `installed_versions()` in `src/tools/mod.rs`

### MCP (`src/mcp/`)

**`McpServer`** (`servers.rs`) - MCP server with id + npx args

**`McpTarget`** (`targets.rs`) - CLI tool that accepts MCP config, with `ConfigMethod`:

- `JsonConfig`: JSON files (Claude, Gemini, Amp, Cursor, Copilot)
- `TomlConfig`: TOML files (Codex)

Adding new MCP server:

1. Add function in `src/mcp/servers.rs`, include in `catalog()`

Adding new MCP target:

1. Add function in `src/mcp/targets.rs`, include in `catalog()`

## Supported Tools

| Tool        | Config File                   |
| ----------- | ----------------------------- |
| Claude Code | `~/.claude.json`              |
| Gemini CLI  | `~/.gemini/settings.json`     |
| Codex CLI   | `~/.codex/config.toml`        |
| Amp         | `~/.config/amp/settings.json` |
| Cursor      | `~/.cursor/mcp.json`          |
| Copilot CLI | `~/.copilot/mcp-config.json`  |
| OpenCode    | `~/.opencode`                 |

## MCP Servers

- linear - Linear issue tracking
- playwright - Browser automation

## Code Style

- `snake_case` functions/variables, `PascalCase` types
- Return `anyhow::Result` with `.context()` for errors
- `#[tokio::test]` for async tests, mock HTTP in tests
- Concise imperative commit messages
