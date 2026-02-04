use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Copilot CLI",
        InstallMethod::Npm("@github/copilot".to_string()),
        vec!["copilot".to_string(), "--version".to_string()],
    )
    .with_binary_name("copilot")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("copilot", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()))
        .map(|s| {
            // Extract version from "GitHub Copilot CLI 0.0.399."
            s.split_whitespace()
                .last()
                .map(|v| v.trim_end_matches('.').to_string())
                .unwrap_or(s)
        });
    ToolVersion::new("Copilot CLI")
        .with_installed(installed)
        .with_identifier("copilot")
}
