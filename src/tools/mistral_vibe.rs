use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Mistral Vibe",
        InstallMethod::Bootstrap("https://mistral.ai/vibe/install.sh".to_string()),
        vec!["vibe".to_string(), "--version".to_string()],
    )
    .with_binary_name("vibe")
    .with_config_dir(".vibe")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("vibe", &["--version"]);
    ToolVersion::new("Mistral Vibe")
        .with_installed(installed)
        .with_identifier("vibe")
}
