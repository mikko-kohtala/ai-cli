# ai-cli

Manage AI CLI tools and MCP servers from one place.

## Requirements

- macOS or Linux
- Node.js (for NPM-based tools)
- Rust toolchain (to build)

## Installation

```bash
git clone https://github.com/mikko-kohtala/ai-cli.git
cd ai-cli
make install
```

## Usage

### Manage AI CLI Tools

```bash
ai-cli apps                  # list installed tools
ai-cli apps check            # check for updates
ai-cli apps install          # install a tool
ai-cli apps update           # update a tool
ai-cli apps uninstall        # uninstall a tool
```

### Manage MCP Servers

```bash
ai-cli mcp                   # list MCP server status
ai-cli mcp enable linear     # enable Linear server
ai-cli mcp disable linear    # disable Linear server
ai-cli mcp doctor            # show config file paths
```

## Supported Tools

- Amp
- Claude Code
- Cline CLI
- Codex CLI
- Copilot CLI
- Cursor CLI
- Factory CLI
- Gemini CLI
- Kilo Code CLI
- OpenCode

## MCP Servers

- **Linear** - issue tracking
- **Playwright** - browser automation
