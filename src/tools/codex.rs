use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Codex CLI",
        InstallMethod::Brew("codex".to_string()),
        vec!["codex".to_string(), "--version".to_string()],
    )
    .with_binary_name("codex")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("codex", &["--version"])
        .map(|v| v.trim_start_matches("codex-cli").trim().to_string());
    ToolVersion::new("Codex CLI")
        .with_installed(installed)
        .with_identifier("codex")
}
