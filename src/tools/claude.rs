use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Claude Code",
        InstallMethod::Npm("@anthropic-ai/claude-code".to_string()),
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
