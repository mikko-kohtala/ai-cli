use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Claude Code",
        InstallMethod::Bootstrap(
            "https://storage.googleapis.com/claude-code-dist-86c565f3-f756-42ad-8dfa-d59b1c096819/claude-code-releases/bootstrap.sh"
                .to_string(),
        ),
        vec!["claude".to_string(), "--version".to_string()],
    )
    .with_binary_name("claude")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("claude", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.replace(" (Claude Code)", "")));
    ToolVersion::new("Claude Code")
        .with_installed(installed)
        .with_identifier("claude")
}
