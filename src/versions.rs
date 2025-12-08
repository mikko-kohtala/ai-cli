use std::collections::HashMap;

use colored::*;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;

use crate::tools::ToolVersion;

#[derive(Deserialize)]
struct NpmPackageInfo {
    #[serde(rename = "dist-tags")]
    dist_tags: NpmDistTags,
}

#[derive(Deserialize)]
struct NpmDistTags {
    latest: String,
}

async fn get_factory_cli_latest() -> Option<String> {
    let script = reqwest::get("https://app.factory.ai/cli")
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    script
        .lines()
        .find_map(|line| line.trim().strip_prefix("VER=").map(|value| value.trim()))
        .map(|value| value.trim_matches(|c| c == '"' || c == '\'').to_string())
}

async fn fetch_npm_latest(url: &str) -> Option<String> {
    let response = reqwest::get(url).await.ok()?;
    let info: NpmPackageInfo = response.json().await.ok()?;
    Some(info.dist_tags.latest)
}

async fn get_npm_latest(package: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package);
    fetch_npm_latest(&url).await
}

pub fn is_newer_version(latest: &str, installed: &str) -> bool {
    // Extract numeric parts from version strings
    let parse_version = |v: &str| -> Vec<u32> {
        v.trim_start_matches('v')
            .split('.')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    };

    let latest_parts = parse_version(latest);
    let installed_parts = parse_version(installed);

    // Compare version parts
    for i in 0..latest_parts.len().max(installed_parts.len()) {
        let latest_part = latest_parts.get(i).copied().unwrap_or(0);
        let installed_part = installed_parts.get(i).copied().unwrap_or(0);

        if latest_part > installed_part {
            return true;
        } else if latest_part < installed_part {
            return false;
        }
    }

    false
}

pub async fn check_latest_versions(tools: &mut [ToolVersion]) {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    spinner.set_message("Fetching versions...");
    let sources = vec![
        (
            "Claude Code",
            tokio::spawn(get_npm_latest("@anthropic-ai/claude-code")),
        ),
        ("Amp", tokio::spawn(get_npm_latest("@sourcegraph/amp"))),
        ("Codex CLI", tokio::spawn(get_npm_latest("@openai/codex"))),
        (
            "Copilot CLI",
            tokio::spawn(get_npm_latest("@github/copilot")),
        ),
        (
            "Gemini CLI",
            tokio::spawn(get_npm_latest("@google/gemini-cli")),
        ),
        ("Cline CLI", tokio::spawn(get_npm_latest("cline"))),
        (
            "Kilo Code CLI",
            tokio::spawn(get_npm_latest("@kilocode/cli")),
        ),
        ("OpenCode", tokio::spawn(get_npm_latest("opencode-ai"))),
        ("Factory CLI", tokio::spawn(get_factory_cli_latest())),
    ];

    let resolved = join_all(
        sources
            .into_iter()
            .map(|(name, handle)| async move { (name, handle.await.ok().and_then(|r| r)) }),
    )
    .await;

    let latest_map: HashMap<_, _> = resolved.into_iter().collect();

    for tool in tools.iter_mut() {
        if let Some(latest) = latest_map.get(tool.name.as_str()) {
            tool.latest = latest.clone();
        }
    }

    spinner.finish_and_clear();
}

pub fn print_version(tool: &ToolVersion, check_latest: bool, label_width: usize, id_width: usize) {
    let status = match &tool.installed {
        Some(version) => {
            let version_str = version.to_string();
            if check_latest {
                if let Some(latest) = &tool.latest {
                    if version.contains(latest) || latest.contains(version) {
                        version_str.green().to_string()
                    } else if is_newer_version(latest, version) {
                        format!(
                            "{} → {} available",
                            version_str.yellow(),
                            latest.bright_blue()
                        )
                    } else {
                        version_str.green().to_string()
                    }
                } else {
                    version_str.green().to_string()
                }
            } else {
                version_str.green().to_string()
            }
        }
        None => {
            if check_latest && tool.latest.is_some() {
                format!(
                    "{} ({})",
                    "not installed".red(),
                    tool.latest.as_ref().unwrap().bright_blue()
                )
            } else {
                "not installed".red().to_string()
            }
        }
    };

    let name_padding = label_width.saturating_sub(tool.name.len());
    let name_spacer = " ".repeat(name_padding + 1);
    let identifier = tool.identifier.as_deref().unwrap_or(tool.name.as_str());
    let id_padding = id_width.saturating_sub(identifier.len());
    let id_spacer = " ".repeat(id_padding + 1);

    println!(
        "{}{}{}{}{}",
        format!("{}:", tool.name).bold(),
        name_spacer,
        identifier.bright_black(),
        id_spacer,
        status
    );
}

#[cfg(test)]
mod tests {
    use super::fetch_npm_latest;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn it_fetches_latest_from_npm_dist_tags() {
        let server = MockServer::start_async().await;
        let _mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/@github/copilot");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(r#"{"dist-tags":{"latest":"0.0.357"}}"#);
            })
            .await;

        let latest = fetch_npm_latest(&format!("{}/@github/copilot", server.base_url())).await;
        assert_eq!(latest.as_deref(), Some("0.0.357"));
    }
}
