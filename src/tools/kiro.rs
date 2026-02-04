use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Kiro CLI",
        InstallMethod::Bootstrap("https://cli.kiro.dev/install".to_string()),
        vec!["kiro-cli".to_string(), "--version".to_string()],
    )
    .with_binary_name("kiro-cli")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("kiro-cli", &["--version"]).map(|s| s.replace("kiro-cli ", ""));
    ToolVersion::new("Kiro CLI")
        .with_installed(installed)
        .with_identifier("kiro-cli")
}
