use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Gemini CLI",
        InstallMethod::Npm("@google/gemini-cli".to_string()),
        vec!["gemini".to_string(), "--version".to_string()],
    )
    .with_binary_name("gemini")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("gemini", &["--version"]);
    ToolVersion::new("Gemini CLI")
        .with_installed(installed)
        .with_identifier("gemini")
}
