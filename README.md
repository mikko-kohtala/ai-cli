# ai-cli-apps

A Rust-based CLI tool to manage AI development tools from the command line.

## Installation

```bash
make install
```

## Commands

### Check Versions

```bash
ai-cli-apps          # Show installed versions
ai-cli-apps list     # Show installed versions (alias)
ai-cli-apps check    # Show installed versions (alias)
```

### Install Tools

```bash
# Interactive mode - select from a menu
ai-cli-apps install
ai-cli-apps add       # Alias for install

# Direct installation - specify tool name
ai-cli-apps install claude
ai-cli-apps add claude
```

### Uninstall Tools

```bash
# Interactive mode - select from a menu
ai-cli-apps uninstall
ai-cli-apps remove    # Alias for uninstall

# Direct uninstallation - specify tool name
ai-cli-apps uninstall claude
ai-cli-apps remove claude
```

## Supported Tools

- **Amp**
- **Claude Code**
- **Code CLIx**
- **Cursor CLI**
- **Copilot CLI**
- **Kilo Code CLI**
- **Gemini CLI**
- **Cline CLI**

## Development

```bash
make build    # Build release binary
make clean    # Clean build artifacts
make test     # Run tests
```
