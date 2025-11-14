use super::{InstallMethod, Tool, ToolVersion, command_output};

pub fn definition() -> Tool {
    Tool::new(
        "Factory CLI",
        InstallMethod::Bootstrap("https://app.factory.ai/cli".to_string()),
        vec!["droid".to_string(), "--version".to_string()],
    )
    .with_binary_name("droid")
    .with_config_dir(".factory")
}

pub fn installed_version() -> ToolVersion {
    let installed = command_output("droid", &["--version"]).and_then(|output| {
        output
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }

                let version_candidate = trimmed.trim_start_matches('v');
                if !version_candidate.is_empty()
                    && version_candidate
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '.')
                {
                    Some(version_candidate.to_string())
                } else {
                    None
                }
            })
            .last()
    });
    ToolVersion::new("Factory CLI")
        .with_installed(installed)
        .with_identifier("droid")
}
