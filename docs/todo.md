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
| Version Check            | ✅ Implemented     | `opencode --version`                                                                                       |
| Current Version          | ✅ Implemented     | Parse CLI output                                                                                           |
| Latest Available Version | ✅ Implemented     | Homebrew formula `opencode`                                                                                |
| Install                  | ✅ Implemented     | `curl -fsSL https://opencode.ai/install \| bash`                                                           |
| Uninstall                | ✅ Implemented     | Remove `~/.opencode/bin/opencode` + optional config                                                        |
| Upgrade                  | ✅ Implemented     | Re-run install script                                                                                      |

## Factory CLI

**Documentation**: [Docs](https://factory.ai/product/cli)

| Operation                | Status             | Method                                        |
| ------------------------ | ------------------ | --------------------------------------------- |
| Version Check            | ✅ Implemented     | `droid --version`                             |
| Current Version          | ✅ Implemented     | Parse CLI output                              |
| Latest Available Version | ✅ Implemented     | Parse `VER=` from install script              |
| Install                  | ✅ Implemented     | `curl -fsSL https://app.factory.ai/cli \| sh` |
| Uninstall                | ✅ Implemented     | Remove `~/.local/bin/droid` + optional config |
| Upgrade                  | ✅ Implemented     | Re-run install script                         |

## Next Steps

1. Add automated smoke tests for new installers and uninstallers
2. Monitor upstream release feeds for format changes (Factory CLI script, Brew formula metadata)
