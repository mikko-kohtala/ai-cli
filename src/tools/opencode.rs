use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "OpenCode",
        InstallMethod::Bootstrap("https://opencode.ai/install".to_string()),
        vec!["opencode".to_string(), "--version".to_string()],
    )
    .with_binary_name("opencode")
    .with_config_dir(".opencode")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("opencode", &["--version"]);
    ToolVersion::new("OpenCode")
        .with_installed(installed)
        .with_identifier("opencode")
}
