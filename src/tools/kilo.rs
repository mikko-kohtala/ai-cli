use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Kilo Code CLI",
        InstallMethod::Npm("@kilocode/cli".to_string()),
        vec!["kilo".to_string(), "--version".to_string()],
    )
    .with_binary_name("kilo")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("kilo", &["--version"]);
    ToolVersion::new("Kilo Code CLI")
        .with_installed(installed)
        .with_identifier("kilo")
}
