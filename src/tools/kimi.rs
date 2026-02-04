use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Kimi CLI",
        InstallMethod::Bootstrap("https://code.kimi.com/install.sh".to_string()),
        vec!["kimi".to_string(), "--version".to_string()],
    )
    .with_binary_name("kimi")
    .with_config_dir(".kimi")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("kimi", &["--version"]).map(|s| {
        // Extract version from "kimi, version 1.5"
        s.split("version")
            .nth(1)
            .map(|v| v.trim().to_string())
            .unwrap_or(s)
    });
    ToolVersion::new("Kimi CLI")
        .with_installed(installed)
        .with_identifier("kimi")
}
