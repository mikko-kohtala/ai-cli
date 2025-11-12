use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use inquire::MultiSelect;
use serde::Deserialize;
use std::process::Command;

#[derive(Parser)]
#[command(name = "ai-cli-apps")]
#[command(
    about = "Check and manage AI CLI tools versions",
    long_about = "Check and manage AI CLI tools versions\n\nSupported tools:\n  Claude Code (claude)\n  Amp (amp)\n  Codex (codex)\n  Cursor (cursor)\n  Copilot CLI (copilot)\n  Kilo (kilo)\n  Gemini (gemini)\n  Cline (cline)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check latest versions available
    Check,
    /// Upgrade AI CLI tools (optionally specify tool name, e.g., 'amp')
    Upgrade {
        /// Optional tool name to upgrade directly (e.g., 'amp')
        tool: Option<String>,
    },
    /// Update AI CLI tools (alias for upgrade)
    Update {
        /// Optional tool name to update directly (e.g., 'amp')
        tool: Option<String>,
    },
    /// Install AI CLI tools (optionally specify tool name, e.g., 'claude')
    Install {
        /// Optional tool name to install directly (e.g., 'claude')
        tool: Option<String>,
    },
    /// Install AI CLI tools (alias for install)
    Add {
        /// Optional tool name to install directly (e.g., 'claude')
        tool: Option<String>,
    },
    /// Uninstall AI CLI tools (optionally specify tool name, e.g., 'claude')
    Uninstall {
        /// Optional tool name to uninstall directly (e.g., 'claude')
        tool: Option<String>,
        /// Remove config directory (will ask for confirmation unless --force is used)
        #[arg(long)]
        remove_config: bool,
        /// Skip all confirmation prompts
        #[arg(long)]
        force: bool,
    },
    /// Uninstall AI CLI tools (alias for uninstall)
    Remove {
        /// Optional tool name to uninstall directly (e.g., 'claude')
        tool: Option<String>,
        /// Remove config directory (will ask for confirmation unless --force is used)
        #[arg(long)]
        remove_config: bool,
        /// Skip all confirmation prompts
        #[arg(long)]
        force: bool,
    },
    /// List installed AI CLI tools (alias for default command)
    List,
}

#[derive(Debug, Clone)]
enum InstallMethod {
    Npm(String),       // NPM package name
    GitHub(String),    // GitHub repo (owner/repo)
    Bootstrap(String), // Bootstrap script URL
    Amp(String),       // Amp installer script URL
    Custom(String),    // Custom installation message
}

#[derive(Debug, Clone)]
struct Tool {
    name: String,
    install_method: InstallMethod,
    check_command: Vec<String>,
    binary_name: Option<String>,
}

impl Tool {
    fn new(name: &str, install_method: InstallMethod, check_command: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            install_method,
            check_command,
            binary_name: None,
        }
    }

    fn with_binary_name(mut self, binary_name: &str) -> Self {
        self.binary_name = Some(binary_name.to_string());
        self
    }

    fn is_installed(&self) -> bool {
        if self.check_command.is_empty() {
            return false;
        }
        Command::new(&self.check_command[0])
            .args(&self.check_command[1..])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[derive(Debug)]
struct ToolVersion {
    name: String,
    installed: Option<String>,
    latest: Option<String>,
}

impl ToolVersion {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            installed: None,
            latest: None,
        }
    }

    fn with_installed(mut self, version: Option<String>) -> Self {
        self.installed = version;
        self
    }
}

#[derive(Deserialize)]
struct NpmPackageInfo {
    version: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

fn check_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn get_claude_version() -> ToolVersion {
    let installed = check_command("claude", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Claude Code").with_installed(installed)
}

fn get_amp_version() -> ToolVersion {
    let installed =
        check_command("amp", &["--version"]).and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Amp").with_installed(installed)
}

fn get_codex_version() -> ToolVersion {
    let installed = check_command("codex", &["--version"]);
    ToolVersion::new("Codex").with_installed(installed)
}

fn get_cursor_version() -> ToolVersion {
    let installed = check_command(
        "defaults",
        &[
            "read",
            "/Applications/Cursor.app/Contents/Info.plist",
            "CFBundleShortVersionString",
        ],
    );
    ToolVersion::new("Cursor").with_installed(installed)
}

fn get_copilot_version() -> ToolVersion {
    let installed = check_command("copilot", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Copilot").with_installed(installed)
}

fn get_kilo_version() -> ToolVersion {
    let installed = check_command("kilo", &["--version"]);
    ToolVersion::new("Kilo").with_installed(installed)
}

fn get_gemini_version() -> ToolVersion {
    let installed = check_command("gemini", &["--version"]);
    ToolVersion::new("Gemini").with_installed(installed)
}

fn get_cline_version() -> ToolVersion {
    let installed = check_command("cline", &["version"]).and_then(|output| {
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
    ToolVersion::new("Cline").with_installed(installed)
}

async fn get_npm_latest(package: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package);
    let response = reqwest::get(&url).await.ok()?;
    let info: NpmPackageInfo = response.json().await.ok()?;
    Some(info.version)
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

fn print_version(tool: &ToolVersion, check_latest: bool) {
    let status = match &tool.installed {
        Some(version) => {
            let version_str = format!("{}", version);
            if check_latest {
                if let Some(latest) = &tool.latest {
                    if version.contains(latest) || latest.contains(version) {
                        format!("{} ✓", version_str.green())
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

    println!("{:12} {}", format!("{}:", tool.name).bold(), status);
}

async fn get_all_versions() -> Vec<ToolVersion> {
    vec![
        get_claude_version(),
        get_amp_version(),
        get_codex_version(),
        get_cursor_version(),
        get_copilot_version(),
        get_kilo_version(),
        get_gemini_version(),
        get_cline_version(),
    ]
}

async fn check_latest_versions(tools: &mut [ToolVersion]) {
    println!("{}", "Checking latest versions...".cyan());

    // Fetch latest versions in parallel
    let handles = vec![
        tokio::spawn(get_github_latest("anthropics/anthropic-quickstarts")),
        tokio::spawn(get_npm_latest("@openai/codex")),
        tokio::spawn(get_npm_latest("@github/copilot")),
        tokio::spawn(get_npm_latest("@google/gemini-cli")),
        tokio::spawn(get_github_latest("cline/cline")),
        tokio::spawn(get_github_latest("Kilo-Org/kilocode")),
    ];

    let results = futures::future::join_all(handles).await;

    // Update tools with latest versions
    for tool in tools.iter_mut() {
        tool.latest = match tool.name.as_str() {
            "Claude Code" => results[0].as_ref().ok().and_then(|r| r.clone()),
            "Codex" => results[1].as_ref().ok().and_then(|r| r.clone()),
            "Copilot" => results[2].as_ref().ok().and_then(|r| r.clone()),
            "Gemini" => results[3].as_ref().ok().and_then(|r| r.clone()),
            "Cline" => results[4].as_ref().ok().and_then(|r| r.clone()),
            "Kilo" => results[5].as_ref().ok().and_then(|r| r.clone()),
            _ => None,
        };
    }
}

fn get_all_tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "Claude Code",
            InstallMethod::Bootstrap(
                "https://storage.googleapis.com/claude-code-dist-86c565f3-f756-42ad-8dfa-d59b1c096819/claude-code-releases/bootstrap.sh".to_string()
            ),
            vec!["claude".to_string(), "--version".to_string()],
        ).with_binary_name("claude"),
        Tool::new(
            "Amp",
            InstallMethod::Amp("https://ampcode.com/install.sh".to_string()),
            vec!["amp".to_string(), "--version".to_string()],
        ).with_binary_name("amp"),
        Tool::new(
            "Codex",
            InstallMethod::Npm("@openai/codex".to_string()),
            vec!["codex".to_string(), "--version".to_string()],
        ).with_binary_name("codex"),
        Tool::new(
            "Cursor",
            InstallMethod::Custom("Visit https://cursor.sh to download".to_string()),
            vec![
                "defaults".to_string(),
                "read".to_string(),
                "/Applications/Cursor.app/Contents/Info.plist".to_string(),
                "CFBundleShortVersionString".to_string(),
            ],
        ),
        Tool::new(
            "Copilot",
            InstallMethod::Npm("@github/copilot".to_string()),
            vec!["copilot".to_string(), "--version".to_string()],
        ).with_binary_name("copilot"),
        Tool::new(
            "Kilo",
            InstallMethod::GitHub("Kilo-Org/kilocode".to_string()),
            vec!["kilo".to_string(), "--version".to_string()],
        ).with_binary_name("kilo"),
        Tool::new(
            "Gemini",
            InstallMethod::Npm("@google/gemini-cli".to_string()),
            vec!["gemini".to_string(), "--version".to_string()],
        ).with_binary_name("gemini"),
        Tool::new(
            "Cline",
            InstallMethod::GitHub("cline/cline".to_string()),
            vec!["cline".to_string(), "version".to_string()],
        ).with_binary_name("cline"),
    ]
}

async fn run_install_script(url: &str, temp_filename: &str, description: &str) -> Result<()> {
    println!("{} Downloading {}...", "→".cyan(), description);

    let script = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to download {}", description))?
        .text()
        .await
        .with_context(|| format!("Failed to read {}", description))?;

    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join(temp_filename);
    std::fs::write(&script_path, script)
        .with_context(|| format!("Failed to write {}", description))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
    }

    println!("{} Running {}...", "→".cyan(), description);
    println!();

    let status = Command::new("bash")
        .arg(&script_path)
        .status()
        .context("Failed to run install script")?;

    let _ = std::fs::remove_file(&script_path);

    println!();
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Installation failed - see output above for details");
    }
}

async fn install_tool(tool: &Tool) -> Result<()> {
    println!("Installing {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Bootstrap(url) => {
            run_install_script(url, "bootstrap.sh", "bootstrap script").await?;
            println!("{} {} installed successfully!", "✓".green(), tool.name);
        }
        InstallMethod::Amp(url) => {
            run_install_script(url, "amp_install.sh", "Amp installer").await?;
            println!("{} {} installed successfully!", "✓".green(), tool.name);
        }
        InstallMethod::Npm(package) => {
            let status = Command::new("npm")
                .args(&["install", "-g", package])
                .status()
                .context("Failed to run npm install")?;

            if status.success() {
                println!("{} {} installed successfully!", "✓".green(), tool.name);
            } else {
                anyhow::bail!("npm install failed for {}", tool.name);
            }
        }
        InstallMethod::GitHub(repo) => {
            println!("{} Fetching latest release from GitHub...", "→".cyan());

            // Get the latest release info
            let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("User-Agent", "ai-cli-apps")
                .send()
                .await
                .context("Failed to fetch GitHub release")?;

            let release: serde_json::Value = response.json().await?;
            let tag_name = release["tag_name"]
                .as_str()
                .context("No tag_name in release")?;

            // Try to find a binary asset for the current platform
            let assets = release["assets"]
                .as_array()
                .context("No assets in release")?;

            // Look for macOS binary (darwin, macos) or generic binary
            let os_keywords = ["darwin", "macos", "mac", "universal"];
            let binary_asset = assets.iter().find(|asset| {
                if let Some(name) = asset["name"].as_str() {
                    let name_lower = name.to_lowercase();
                    os_keywords
                        .iter()
                        .any(|keyword| name_lower.contains(keyword))
                        && !name_lower.ends_with(".sha256")
                        && !name_lower.ends_with(".txt")
                } else {
                    false
                }
            });

            if let Some(asset) = binary_asset {
                let download_url = asset["browser_download_url"]
                    .as_str()
                    .context("No download URL")?;
                let asset_name = asset["name"].as_str().unwrap_or("binary");

                println!("{} Downloading {}...", "→".cyan(), asset_name);

                // Download the binary
                let binary_data = reqwest::get(download_url).await?.bytes().await?;

                // Determine installation path
                let install_dir = std::path::Path::new("/usr/local/bin");
                let binary_name = tool.binary_name.as_ref().unwrap_or(&tool.name);
                let install_path = install_dir.join(binary_name);

                // Write the binary
                std::fs::write(&install_path, &binary_data).context("Failed to write binary")?;

                // Make it executable
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&install_path)?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&install_path, perms)?;
                }

                println!(
                    "{} {} installed successfully to {}!",
                    "✓".green(),
                    tool.name,
                    install_path.display()
                );
            } else {
                println!("{} No suitable binary found for your platform.", "✗".red());
                println!(
                    "Please visit https://github.com/{}/releases/tag/{}",
                    repo, tag_name
                );
                anyhow::bail!("No binary available for platform");
            }
        }
        InstallMethod::Custom(message) => {
            println!("{}", message.yellow());
            anyhow::bail!("Custom installation required");
        }
    }

    Ok(())
}

async fn uninstall_tool(tool: &Tool, remove_config: bool, force: bool) -> Result<()> {
    println!("Uninstalling {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Bootstrap(_) => {
            // Handle Claude-specific uninstallation
            let home = std::env::var("HOME").context("HOME environment variable not set")?;
            let binary_name = tool.binary_name.as_ref().unwrap_or(&tool.name);

            // Remove symlink in ~/.local/bin
            let symlink_path = std::path::Path::new(&home)
                .join(".local")
                .join("bin")
                .join(binary_name);

            // Remove versions directory
            let versions_path = std::path::Path::new(&home)
                .join(".local")
                .join("share")
                .join(binary_name)
                .join("versions");

            // Remove config directory
            let config_path = std::path::Path::new(&home).join(format!(".{}", binary_name));

            let mut removed_items = Vec::new();

            if symlink_path.exists() {
                std::fs::remove_file(&symlink_path).context("Failed to remove binary symlink")?;
                removed_items.push(format!("binary: {}", symlink_path.display()));
            }

            if versions_path.exists() {
                std::fs::remove_dir_all(&versions_path.parent().unwrap())
                    .context("Failed to remove versions directory")?;
                removed_items.push(format!(
                    "versions: {}",
                    versions_path.parent().unwrap().display()
                ));
            }

            if config_path.exists() {
                println!(
                    "{} Config directory found at: {}",
                    "→".cyan(),
                    config_path.display()
                );

                if remove_config {
                    let should_remove = if force {
                        true
                    } else {
                        println!(
                            "{} Remove config directory? (contains settings and history) [y/N]",
                            "?".yellow()
                        );
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;
                        input.trim().eq_ignore_ascii_case("y")
                    };

                    if should_remove {
                        std::fs::remove_dir_all(&config_path)
                            .context("Failed to remove config directory")?;
                        removed_items.push(format!("config: {}", config_path.display()));
                    } else {
                        println!("{} Keeping config directory", "→".cyan());
                    }
                } else {
                    println!(
                        "{} Keeping config directory (use --remove-config to remove it)",
                        "→".cyan()
                    );
                }
            }

            if removed_items.is_empty() {
                println!("{} {} not found on system", "!".yellow(), tool.name);
            } else {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
                println!("{} Removed:", "→".cyan());
                for item in removed_items {
                    println!("  - {}", item);
                }
            }
        }
        InstallMethod::Amp(_) => {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .context("HOME environment variable not set")?;
            let home_path = std::path::Path::new(&home);
            let amp_home = home_path.join(".amp");
            let local_bin = home_path.join(".local").join("bin");
            let mut removed_items = Vec::new();

            for shim in ["amp", "amp.bat"] {
                let shim_path = local_bin.join(shim);
                if shim_path.exists() {
                    std::fs::remove_file(&shim_path)
                        .with_context(|| format!("Failed to remove {}", shim_path.display()))?;
                    removed_items.push(format!("shim: {}", shim_path.display()));
                }
            }

            if amp_home.exists() {
                std::fs::remove_dir_all(&amp_home)
                    .context("Failed to remove AMP_HOME directory")?;
                removed_items.push(format!("AMP_HOME: {}", amp_home.display()));
            }

            let config_home = std::env::var("XDG_CONFIG_HOME")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".config"));
            let data_home = std::env::var("XDG_DATA_HOME")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".local").join("share"));
            let cache_home = std::env::var("XDG_CACHE_HOME")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".cache"));

            if remove_config {
                let should_remove = if force {
                    true
                } else {
                    println!(
                        "{} Remove Amp config/cache directories? [y/N]",
                        "?".yellow()
                    );
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    input.trim().eq_ignore_ascii_case("y")
                };

                if should_remove {
                    for path in [
                        config_home.join("amp"),
                        data_home.join("amp"),
                        cache_home.join("amp"),
                    ] {
                        if path.exists() {
                            let metadata = std::fs::metadata(&path)?;
                            if metadata.is_file() {
                                std::fs::remove_file(&path).with_context(|| {
                                    format!("Failed to remove {}", path.display())
                                })?;
                            } else {
                                std::fs::remove_dir_all(&path).with_context(|| {
                                    format!("Failed to remove {}", path.display())
                                })?;
                            }
                            removed_items.push(format!("config/data/cache: {}", path.display()));
                        }
                    }
                } else {
                    println!("{} Keeping Amp config/cache directories", "→".cyan());
                }
            } else {
                println!(
                    "{} Keeping Amp config/cache directories (use --remove-config to delete them)",
                    "→".cyan()
                );
            }

            if removed_items.is_empty() {
                println!("{} Amp files not found on system", "!".yellow());
            } else {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
                println!("{} Removed:", "→".cyan());
                for item in removed_items {
                    println!("  - {}", item);
                }
                println!(
                    "{} Remove any PATH entries for ~/.local/bin/amp in your shell rc files.",
                    "→".cyan()
                );
            }
        }
        InstallMethod::Npm(package) => {
            let status = Command::new("npm")
                .args(&["uninstall", "-g", package])
                .status()
                .context("Failed to run npm uninstall")?;

            if status.success() {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
            } else {
                anyhow::bail!("npm uninstall failed for {}", tool.name);
            }
        }
        InstallMethod::GitHub(_) => {
            let binary_name = tool.binary_name.as_ref().unwrap_or(&tool.name);
            let install_path = std::path::Path::new("/usr/local/bin").join(binary_name);

            if install_path.exists() {
                std::fs::remove_file(&install_path).context("Failed to remove binary")?;
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
            } else {
                println!(
                    "{} {} binary not found at {}",
                    "!".yellow(),
                    tool.name,
                    install_path.display()
                );
            }
        }
        InstallMethod::Custom(_) => {
            println!(
                "{} {} requires manual uninstallation",
                "!".yellow(),
                tool.name
            );
            println!("Please remove it manually from your system");
        }
    }

    Ok(())
}

async fn upgrade_tool(tool: &Tool) -> Result<()> {
    println!("Upgrading {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Amp(_) => {
            println!("{} Running `amp update`...", "→".cyan());
            let status = Command::new("amp")
                .arg("update")
                .status()
                .context("Failed to run `amp update`")?;

            if status.success() {
                println!("{} {} upgraded successfully!", "✓".green(), tool.name);
                Ok(())
            } else {
                anyhow::bail!("`amp update` failed - see output above for details");
            }
        }
        _ => {
            anyhow::bail!("Upgrade not implemented for {}", tool.name);
        }
    }
}

async fn handle_install_command(tool_name: Option<&str>) -> Result<()> {
    let tools = get_all_tools();

    // If a specific tool is requested, install it directly
    if let Some(name) = tool_name {
        let tool = tools
            .iter()
            .find(|t| {
                t.name.eq_ignore_ascii_case(name)
                    || t.binary_name
                        .as_ref()
                        .map(|b| b.eq_ignore_ascii_case(name))
                        .unwrap_or(false)
            })
            .context(format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                tools
                    .iter()
                    .map(|t| {
                        if let Some(bin) = &t.binary_name {
                            format!("{} ({})", t.name, bin)
                        } else {
                            t.name.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ))?;

        if tool.is_installed() {
            println!("{} {} is already installed!", "✓".green(), tool.name);
            return Ok(());
        }

        install_tool(tool).await?;
        return Ok(());
    }

    // Interactive mode: show multi-select menu
    let mut uninstalled_tools: Vec<&Tool> = tools.iter().filter(|t| !t.is_installed()).collect();
    let installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if uninstalled_tools.is_empty() {
        println!("{}", "All tools are already installed! ✓".green());
        return Ok(());
    }

    // Sort for consistent display
    uninstalled_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to install:".bright_cyan().bold());

    // Create display strings
    let options: Vec<String> = uninstalled_tools
        .iter()
        .map(|t| {
            format!(
                "{} ({})",
                t.name,
                match &t.install_method {
                    InstallMethod::Npm(pkg) => format!("npm: {}", pkg),
                    InstallMethod::GitHub(repo) => format!("github: {}", repo),
                    InstallMethod::Bootstrap(_) => "bootstrap".to_string(),
                    InstallMethod::Amp(_) => "amp installer".to_string(),
                    InstallMethod::Custom(_) => "custom".to_string(),
                }
            )
        })
        .collect();

    // Show installed tools info
    if !installed_tools.is_empty() {
        println!("\n{}", "Already installed:".bright_black());
        for tool in &installed_tools {
            println!("  {} {}", "✓".green(), tool.name.bright_black());
        }
        println!();
    }

    // Multi-select prompt
    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting installation...".bright_cyan());

            // Find selected tools by matching display strings
            for selection in selections {
                if let Some(tool) = uninstalled_tools
                    .iter()
                    .find(|t| selection.starts_with(&t.name))
                {
                    if let Err(e) = install_tool(tool).await {
                        println!("{} Failed to install {}: {}", "✗".red(), tool.name, e);
                    }
                }
            }

            println!("\n{}", "Installation complete!".green().bold());
        }
        Ok(_) => {
            println!("{}", "No tools selected.".yellow());
        }
        Err(e) => {
            println!("{} Selection cancelled: {}", "✗".red(), e);
        }
    }

    Ok(())
}

async fn handle_uninstall_command(
    tool_name: Option<&str>,
    remove_config: bool,
    force: bool,
) -> Result<()> {
    let tools = get_all_tools();

    // If a specific tool is requested, uninstall it directly
    if let Some(name) = tool_name {
        let tool = tools
            .iter()
            .find(|t| {
                t.name.eq_ignore_ascii_case(name)
                    || t.binary_name
                        .as_ref()
                        .map(|b| b.eq_ignore_ascii_case(name))
                        .unwrap_or(false)
            })
            .context(format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                tools
                    .iter()
                    .map(|t| {
                        if let Some(bin) = &t.binary_name {
                            format!("{} ({})", t.name, bin)
                        } else {
                            t.name.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ))?;

        if !tool.is_installed() {
            println!("{} {} is not installed!", "!".yellow(), tool.name);
            return Ok(());
        }

        uninstall_tool(tool, remove_config, force).await?;
        return Ok(());
    }

    // Interactive mode: show multi-select menu
    let mut installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if installed_tools.is_empty() {
        println!("{}", "No tools are currently installed.".yellow());
        return Ok(());
    }

    // Sort for consistent display
    installed_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to uninstall:".bright_cyan().bold());

    // Create display strings
    let options: Vec<String> = installed_tools.iter().map(|t| t.name.clone()).collect();

    // Multi-select prompt
    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting uninstallation...".bright_cyan());

            for selection in selections {
                if let Some(tool) = installed_tools.iter().find(|t| t.name == selection) {
                    if let Err(e) = uninstall_tool(tool, remove_config, force).await {
                        println!("{} Failed to uninstall {}: {}", "✗".red(), tool.name, e);
                    }
                }
            }

            println!("\n{}", "Uninstallation complete!".green().bold());
        }
        Ok(_) => {
            println!("{}", "No tools selected.".yellow());
        }
        Err(e) => {
            println!("{} Selection cancelled: {}", "✗".red(), e);
        }
    }

    Ok(())
}

async fn handle_upgrade_command(tool_name: Option<&str>) -> Result<()> {
    let tools = get_all_tools();

    let name = if let Some(name) = tool_name {
        name
    } else {
        println!(
            "{} Specify a tool to upgrade, e.g., `ai-cli-apps upgrade amp`.",
            "!".yellow()
        );
        return Ok(());
    };

    let tool = tools
        .iter()
        .find(|t| {
            t.name.eq_ignore_ascii_case(name)
                || t.binary_name
                    .as_ref()
                    .map(|b| b.eq_ignore_ascii_case(name))
                    .unwrap_or(false)
        })
        .with_context(|| {
            format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                tools
                    .iter()
                    .map(|t| {
                        if let Some(bin) = &t.binary_name {
                            format!("{} ({})", t.name, bin)
                        } else {
                            t.name.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    if !tool.is_installed() {
        println!(
            "{} {} is not installed. Run `ai-cli-apps install {}` first.",
            "!".yellow(),
            tool.name,
            name
        );
        return Ok(());
    }

    upgrade_tool(tool).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("\n{}", "🤖 AI Tools Manager".bright_cyan().bold());
    println!("{}\n", "=".repeat(19).bright_cyan());

    match cli.command {
        None | Some(Commands::List) => {
            // Show installed versions
            let mut tools = get_all_versions().await;
            check_latest_versions(&mut tools).await;

            // Separate installed and not installed
            let installed: Vec<_> = tools.iter().filter(|t| t.installed.is_some()).collect();
            let not_installed: Vec<_> = tools.iter().filter(|t| t.installed.is_none()).collect();

            // Show installed tools first
            if !installed.is_empty() {
                println!("{}", "Installed:".bright_green().bold());
                for tool in &installed {
                    print_version(tool, true);
                }
            }

            // Show not installed tools
            if !not_installed.is_empty() {
                if !installed.is_empty() {
                    println!();
                }
                println!("{}", "Not Installed:".bright_black().bold());
                for tool in &not_installed {
                    print_version(tool, true);
                }
            }
        }
        Some(Commands::Check) => {
            // Check latest versions
            let mut tools = get_all_versions().await;
            check_latest_versions(&mut tools).await;
            println!();
            for tool in &tools {
                print_version(tool, true);
            }
        }
        Some(Commands::Upgrade { tool }) | Some(Commands::Update { tool }) => {
            handle_upgrade_command(tool.as_deref()).await?;
        }
        Some(Commands::Install { tool }) | Some(Commands::Add { tool }) => {
            handle_install_command(tool.as_deref()).await?;
        }
        Some(Commands::Uninstall {
            tool,
            remove_config,
            force,
        })
        | Some(Commands::Remove {
            tool,
            remove_config,
            force,
        }) => {
            handle_uninstall_command(tool.as_deref(), remove_config, force).await?;
        }
    }

    println!();
    Ok(())
}
