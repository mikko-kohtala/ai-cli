use std::{collections::HashMap, process::Command};

use colored::*;
use futures::future::join_all;
use serde::Deserialize;
use tokio::task;

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

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

#[derive(Deserialize)]
struct BrewInfo {
    formulae: Vec<BrewFormula>,
    casks: Vec<BrewCask>,
}

#[derive(Deserialize)]
struct BrewFormula {
    versions: BrewVersions,
}

#[derive(Deserialize)]
struct BrewVersions {
    stable: Option<String>,
}

#[derive(Deserialize)]
struct BrewCask {
    version: String,
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

async fn get_github_latest(repo: &str) -> Option<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "ai-cli-apps")
        .send()
        .await
        .ok()?;
    let release: GitHubRelease = response.json().await.ok()?;
    Some(release.tag_name)
}

fn update_brew() -> bool {
    Command::new("brew")
        .args(["update"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

async fn get_brew_latest(formula: &str) -> Option<String> {
    let formula = formula.to_string();
    task::spawn_blocking(move || {
        let output = Command::new("brew")
            .args(["info", "--json=v2", &formula])
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let info: BrewInfo = serde_json::from_slice(&output.stdout).ok()?;

        // Check formulae first
        if let Some(formula_version) = info
            .formulae
            .into_iter()
            .next()
            .and_then(|f| f.versions.stable)
        {
            return Some(formula_version);
        }

        // Fall back to casks
        info.casks.into_iter().next().map(|c| c.version)
    })
    .await
    .ok()
    .flatten()
}

pub async fn check_latest_versions(tools: &mut [ToolVersion]) {
    println!("{}", "Checking latest versions...".cyan());

    // Update Homebrew package database before checking versions
    task::spawn_blocking(|| {
        if update_brew() {
            println!("{}", "Updated Homebrew package database".dimmed());
        }
    })
    .await
    .ok();

    let sources = vec![
        (
            "Claude Code",
            tokio::spawn(get_github_latest("anthropics/anthropic-quickstarts")),
        ),
        ("Amp", tokio::spawn(get_npm_latest("@sourcegraph/amp"))),
        ("Codex CLI", tokio::spawn(get_brew_latest("codex"))),
        (
            "Copilot CLI",
            tokio::spawn(get_npm_latest("@github/copilot")),
        ),
        ("Gemini CLI", tokio::spawn(get_brew_latest("gemini-cli"))),
        ("Cline CLI", tokio::spawn(get_npm_latest("cline"))),
        (
            "Kilo Code CLI",
            tokio::spawn(get_npm_latest("@kilocode/cli")),
        ),
        ("OpenCode", tokio::spawn(get_brew_latest("opencode"))),
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
}

pub fn print_version(tool: &ToolVersion, check_latest: bool, label_width: usize, id_width: usize) {
    let status = match &tool.installed {
        Some(version) => {
            let version_str = version.to_string();
            if check_latest {
                if let Some(latest) = &tool.latest {
                    if version.contains(latest) || latest.contains(version) {
                        version_str.green().to_string()
                    } else {
                        format!(
                            "{} → {} available",
                            version_str.yellow(),
                            latest.bright_blue()
                        )
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
    let identifier = tool
        .identifier
        .as_deref()
        .unwrap_or_else(|| tool.name.as_str());
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
