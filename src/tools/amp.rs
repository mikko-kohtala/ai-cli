use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Amp",
        InstallMethod::Amp("https://ampcode.com/install.sh".to_string()),
        vec!["amp".to_string(), "--version".to_string()],
    )
    .with_binary_name("amp")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("amp", &["--version"]).and_then(|s| {
        s.lines()
            .next()
            .map(|l| l.split(" (released").next().unwrap_or(l).to_string())
    });
    ToolVersion::new("Amp")
        .with_installed(installed)
        .with_identifier("amp")
}
