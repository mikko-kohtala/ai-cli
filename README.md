# ai-cli

A Rust-based CLI tool to manage AI development tools from the command line.

## Installation

```bash
make install
```

## Commands

### Apps - Manage AI CLI Tools

```bash
ai-cli apps              # Show installed versions
ai-cli apps list         # Show installed versions (alias)
ai-cli apps check        # Check latest versions available
ai-cli apps install      # Interactive install
ai-cli apps install amp  # Direct install
ai-cli apps update       # Interactive update
ai-cli apps uninstall    # Interactive uninstall
```

### MCP - Manage MCP Servers

```bash
ai-cli mcp                    # List servers and status
ai-cli mcp list               # List servers and status (alias)
ai-cli mcp enable linear      # Enable Linear MCP server
ai-cli mcp enable playwright  # Enable Playwright MCP server
ai-cli mcp enable all         # Enable all servers
ai-cli mcp disable linear     # Disable Linear MCP server
ai-cli mcp disable all        # Disable all servers
ai-cli mcp doctor             # Show tool config paths
```

## Supported Tools (Apps)

- **Amp**
- **Claude Code**
- **Codex CLI**
- **Cursor CLI**
- **Copilot CLI**
- **Kilo Code CLI**
- **Gemini CLI**
- **Cline CLI**
- **OpenCode**
- **Factory CLI**

## Supported MCP Servers

- **Linear** - Issue tracking integration
- **Playwright** - Browser automation

## Development

```bash
make build    # Build release binary
make clean    # Clean build artifacts
make test     # Run tests
```
