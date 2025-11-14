use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Cline CLI",
        InstallMethod::Npm("cline".to_string()),
        vec!["cline".to_string(), "version".to_string()],
    )
    .with_binary_name("cline")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("cline", &["version"]).and_then(|output| {
        output
            .lines()
            .find(|line| line.contains("Cline CLI Version:"))
            .and_then(|line| {
                line.split_whitespace().nth(3).map(|v| {
                    let core = output
                        .lines()
                        .find(|l| l.contains("Cline Core Version:"))
                        .and_then(|l| l.split_whitespace().nth(3))
                        .unwrap_or("");
                    format!("{} (Core: {})", v, core)
                })
            })
    });
    ToolVersion::new("Cline CLI")
        .with_installed(installed)
        .with_identifier("cline")
}
