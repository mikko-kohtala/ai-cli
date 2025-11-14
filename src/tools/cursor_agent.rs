use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Cursor CLI",
        InstallMethod::Bootstrap("https://cursor.com/install".to_string()),
        vec!["cursor-agent".to_string(), "--version".to_string()],
    )
    .with_binary_name("cursor-agent")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("cursor-agent", &["--version"])
        .and_then(|output| output.lines().next().map(|line| line.to_string()));
    ToolVersion::new("Cursor CLI")
        .with_installed(installed)
        .with_identifier("cursor-agent")
}
