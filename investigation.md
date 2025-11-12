# Investigation Notes

## Amp CLI install/uninstall
- The official installer runs `curl -fsSL https://ampcode.com/install.sh | bash`. It downloads Bun plus `bootstrap.ts`, installs the CLI package into `~/.amp/package`, drops wrappers/binaries into `~/.amp/bin`, and creates a PATH shim at `~/.local/bin/amp`.
- Runtime data lands under the standard XDG folders: configs in `~/.config/amp`, state/tools in `~/.local/share/amp`, and caches/logs in `~/.cache/amp` (including bundled ripgrep).
- A clean uninstall removes the shim (`~/.local/bin/amp` or `.bat`), wipes `~/.amp`, and optionally deletes the config/data/cache directories above. Remove the installer’s `# amp` PATH edits from your shell rc and flush any package-manager installs (e.g. `npm uninstall -g @sourcegraph/amp`) so `which -a amp` reports nothing.
