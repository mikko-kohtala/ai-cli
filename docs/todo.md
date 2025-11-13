# Tool Support Status

This document tracks the implementation status for AI CLI tools that need work.

**⚠️ Note: Currently supports macOS only**

## Legend

- ✅ Implemented
- ⚠️ Partial/Needs work
- ❌ Not implemented
- 🔍 Research needed

## OpenCode

**Documentation**: [Docs](https://opencode.ai/docs)

| Operation                | Status             | Method                                                                                                      |
| ------------------------ | ------------------ | ----------------------------------------------------------------------------------------------------------- |
| Version Check            | 🔍 Research needed | `opencode --version` (likely)                                                                               |
| Current Version          | 🔍 Research needed | Parse CLI output                                                                                            |
| Latest Available Version | 🔍 Research needed | Homebrew or npm registry                                                                                    |
| Install                  | ⚠️ Partial         | `curl -fsSL https://opencode.ai/install \| bash` or `brew install opencode` or `npm install -g opencode-ai` |
| Uninstall                | 🔍 Research needed | `brew uninstall opencode` or `npm uninstall -g opencode-ai`                                                 |
| Upgrade                  | 🔍 Research needed | `brew upgrade opencode` or `npm update -g opencode-ai`                                                      |

## Factory CLI (Droid)

**Documentation**: [Docs](https://factory.ai/product/cli)

| Operation                | Status             | Method                                        |
| ------------------------ | ------------------ | --------------------------------------------- |
| Version Check            | 🔍 Research needed | TBD                                           |
| Current Version          | 🔍 Research needed | TBD                                           |
| Latest Available Version | 🔍 Research needed | TBD                                           |
| Install                  | 🔍 Research needed | `curl -fsSL https://app.factory.ai/cli \| sh` |
| Uninstall                | 🔍 Research needed | TBD                                           |
| Upgrade                  | 🔍 Research needed | TBD                                           |

## Next Steps

1. Verify OpenCode CLI version check command and finalize install/uninstall methods
2. Research Factory CLI (droid) - verify version check, install, uninstall, upgrade commands
3. Implement OpenCode support
4. Implement Factory CLI support
