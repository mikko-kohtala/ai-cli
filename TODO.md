# Tool Support Status

This document tracks the implementation status for each AI CLI tool across different operations.

**⚠️ Note: Currently supports macOS only**

## Legend

- ✅ Implemented
- ⚠️ Partial/Needs work
- ❌ Not implemented
- 🔍 Research needed

## Amp

**Documentation**: [Docs](https://ampcode.com/manual)

| Operation     | Status         | Command/Method                                           |
| ------------- | -------------- | -------------------------------------------------------- |
| Version Check | ✅ Implemented | `amp --version`                                          |
| Install       | ✅ Implemented | `ai-cli-apps install amp` (runs official installer) |
| Uninstall     | ✅ Implemented | `ai-cli-apps uninstall amp`                         |
| Upgrade       | ✅ Implemented | `ai-cli-apps upgrade amp` / `ai-cli-apps update amp` (runs `amp update`)       |

## Claude Code

**Documentation**: [Docs](https://code.claude.com/docs)

| Operation     | Status         | Command/Method                                    |
| ------------- | -------------- | ------------------------------------------------- |
| Version Check | ✅ Implemented | `claude --version`                                |
| Install       | ✅ Implemented | `curl -fsSL https://claude.ai/install.sh \| bash` |
| Uninstall     | ✅ Implemented | Removes binary, versions, and config (optional)   |
| Upgrade       | ⚠️ Partial     | Re-install via bootstrap script                   |

## Codex CLI

**Documentation**: [Docs](https://developers.openai.com/codex/cli/)

| Operation     | Status             | Command/Method    |
| ------------- | ------------------ | ----------------- |
| Version Check | ✅ Implemented     | `codex --version` |
| Install       | ❌ Not implemented |                   |
| Uninstall     | ❌ Not implemented |                   |
| Upgrade       | ❌ Not implemented |                   |

## Cursor CLI

**Documentation**: [Docs](https://cursor.com/docs/cli/overview)

| Operation     | Status             | Command/Method           |
| ------------- | ------------------ | ------------------------ |
| Version Check | ✅ Implemented     | `cursor-agent --version` |
| Install       | ❌ Not implemented |                          |
| Uninstall     | ❌ Not implemented |                          |
| Upgrade       | ❌ Not implemented |                          |

## Copilot CLI

**Documentation**: [Docs](https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli)

| Operation     | Status             | Command/Method      |
| ------------- | ------------------ | ------------------- |
| Version Check | ✅ Implemented     | `copilot --version` |
| Install       | ❌ Not implemented |                     |
| Uninstall     | ❌ Not implemented |                     |
| Upgrade       | ❌ Not implemented |                     |

## Kilo Code CLI

**Documentation**: [Docs](https://kilocode.ai/docs/cli)

| Operation     | Status             | Command/Method   |
| ------------- | ------------------ | ---------------- |
| Version Check | ✅ Implemented     | `kilo --version` |
| Install       | ❌ Not implemented |                  |
| Uninstall     | ❌ Not implemented |                  |
| Upgrade       | ❌ Not implemented |                  |

## Gemini CLI

**Documentation**: [Docs](https://docs.cloud.google.com/gemini/docs/codeassist/gemini-cli)

| Operation     | Status             | Command/Method     |
| ------------- | ------------------ | ------------------ |
| Version Check | ✅ Implemented     | `gemini --version` |
| Install       | ❌ Not implemented |                    |
| Uninstall     | ❌ Not implemented |                    |
| Upgrade       | ❌ Not implemented |                    |

## Cline CLI

**Documentation**: [Docs](https://docs.cline.bot/cline-cli/overview)

| Operation     | Status             | Command/Method  |
| ------------- | ------------------ | --------------- |
| Version Check | ✅ Implemented     | `cline version` |
| Install       | ❌ Not implemented |                 |
| Uninstall     | ❌ Not implemented |                 |
| Upgrade       | ❌ Not implemented |                 |

## Next Steps

1. Research Cursor CLI (`cursor-agent`) - verify version check, install, uninstall, upgrade commands
2. ✅ ~~Research Amp's uninstall process~~ (Done)
3. ✅ ~~Research Claude Code install/uninstall/upgrade process~~ (Done)
4. Implement install/uninstall commands for remaining tools
5. Research upgrade commands for remaining tools
6. Implement upgrade functionality in the CLI
