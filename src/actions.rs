use crate::tools::{self, InstallMethod, Tool};
use crate::versions::{check_latest_versions, is_newer_version};
use anyhow::{Context, Result};
use colored::*;
use futures::future::join_all;
use inquire::MultiSelect;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub async fn handle_install_command(tool_name: Option<&str>) -> Result<()> {
    let tools = tools::catalog();

    if let Some(name) = tool_name {
        let tool = find_tool(&tools, name).with_context(|| {
            format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                format_available_tools(&tools)
            )
        })?;

        if tool.is_installed() {
            println!("{} {} is already installed!", "✓".green(), tool.name);
            return Ok(());
        }

        println!("Installing {}...", tool.name.bright_cyan());
        install_tool(tool.clone()).await?;
        println!("{} {} installed successfully!", "✓".green(), tool.name);
        return Ok(());
    }

    let mut uninstalled_tools: Vec<&Tool> = tools.iter().filter(|t| !t.is_installed()).collect();
    let installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if uninstalled_tools.is_empty() {
        println!("{}", "All tools are already installed! ✓".green());
        return Ok(());
    }

    uninstalled_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to install:".bright_cyan().bold());

    let options: Vec<String> = uninstalled_tools
        .iter()
        .map(|t| {
            format!(
                "{} ({})",
                t.name,
                match &t.install_method {
                    InstallMethod::Npm(pkg) => format!("npm: {}", pkg),
                    InstallMethod::Bootstrap(_) => "bootstrap".to_string(),
                    InstallMethod::Amp(_) => "amp installer".to_string(),
                }
            )
        })
        .collect();

    if !installed_tools.is_empty() {
        println!("\n{}", "Already installed:".bright_black());
        for tool in &installed_tools {
            println!("  {} {}", "✓".green(), tool.name.bright_black());
        }
        println!();
    }

    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting installation...".bright_cyan());

            let handles: Vec<_> = selections
                .iter()
                .filter_map(|selection| {
                    uninstalled_tools
                        .iter()
                        .find(|t| selection.starts_with(&t.name))
                        .map(|t| (*t).clone())
                })
                .map(|tool| {
                    tokio::spawn(async move {
                        let name = tool.name.clone();
                        (name, install_tool(tool).await)
                    })
                })
                .collect();

            let results = join_all(handles).await;
            for result in results {
                let (name, outcome) = result.expect("task panicked");
                match outcome {
                    Ok(_) => println!("{} {} installed successfully!", "✓".green(), name),
                    Err(e) => println!("{} Failed to install {}: {}", "✗".red(), name, e),
                }
            }

            println!("\n{}", "Installation complete!".green().bold());
        }
        Ok(_) => println!("{}", "No tools selected.".yellow()),
        Err(e) => println!("{} Selection cancelled: {}", "✗".red(), e),
    }

    Ok(())
}

pub async fn handle_uninstall_command(
    tool_name: Option<&str>,
    remove_config: bool,
    force: bool,
) -> Result<()> {
    let tools = tools::catalog();

    if let Some(name) = tool_name {
        let tool = find_tool(&tools, name).with_context(|| {
            format!(
                "Tool '{}' not found. Available tools: {}",
                name,
                format_available_tools(&tools)
            )
        })?;

        if !tool.is_installed() {
            println!("{} {} is not installed!", "!".yellow(), tool.name);
            return Ok(());
        }

        uninstall_tool(tool, remove_config, force).await?;
        return Ok(());
    }

    let mut installed_tools: Vec<&Tool> = tools.iter().filter(|t| t.is_installed()).collect();

    if installed_tools.is_empty() {
        println!("{}", "No tools are currently installed.".yellow());
        return Ok(());
    }

    installed_tools.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", "\nSelect tools to uninstall:".bright_cyan().bold());

    let options: Vec<String> = installed_tools.iter().map(|t| t.name.clone()).collect();

    let selected = MultiSelect::new("Tools:", options)
        .with_help_message("↑↓ to move, space to select, enter to confirm")
        .prompt();

    match selected {
        Ok(selections) if !selections.is_empty() => {
            println!("\n{}", "Starting uninstallation...".bright_cyan());

            for selection in selections {
                if let Some(tool) = installed_tools.iter().find(|t| t.name == selection)
                    && let Err(e) = uninstall_tool(tool, remove_config, force).await
                {
                    println!("{} Failed to uninstall {}: {}", "✗".red(), tool.name, e);
                }
            }

            println!("\n{}", "Uninstallation complete!".green().bold());
        }
        Ok(_) => println!("{}", "No tools selected.".yellow()),
        Err(e) => println!("{} Selection cancelled: {}", "✗".red(), e),
    }

    Ok(())
}

pub async fn handle_upgrade_command(tool_name: Option<&str>) -> Result<()> {
    let tools = tools::catalog();

    let Some(name) = tool_name else {
        // Get installed versions and check for latest
        let mut versions = tools::installed_versions();
        check_latest_versions(&mut versions).await;

        // Find tools with updates available
        let updates_available: Vec<(&Tool, &str, &str)> = versions
            .iter()
            .filter_map(|v| {
                let installed = v.installed.as_ref()?;
                let latest = v.latest.as_ref()?;
                if is_newer_version(latest, installed) {
                    // Find the corresponding Tool from catalog
                    let tool = tools.iter().find(|t| t.name == v.name)?;
                    Some((tool, installed.as_str(), latest.as_str()))
                } else {
                    None
                }
            })
            .collect();

        if updates_available.is_empty() {
            println!("{} All tools are up to date!", "✓".green());
            return Ok(());
        }

        // List tools with updates
        println!("Updates available:");
        for (tool, installed, latest) in &updates_available {
            println!(
                "  {} {} {} → {}",
                "•".cyan(),
                tool.name,
                installed.yellow(),
                latest.bright_blue()
            );
        }
        println!();

        // Ask for confirmation
        print!(
            "{} Update {} {}? [y/N] ",
            "?".yellow(),
            updates_available.len(),
            if updates_available.len() == 1 {
                "tool"
            } else {
                "tools"
            }
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }

        // Update tools with available updates
        println!();
        let handles: Vec<_> = updates_available
            .into_iter()
            .map(|(tool, _, _)| {
                let tool = tool.clone();
                tokio::spawn(async move {
                    let name = tool.name.clone();
                    (name, upgrade_tool(tool).await)
                })
            })
            .collect();

        let results = join_all(handles).await;
        for result in results {
            let (name, outcome) = result.expect("task panicked");
            match outcome {
                Ok(_) => println!("{} {} upgraded successfully!", "✓".green(), name),
                Err(e) => println!("{} Failed to upgrade {}: {}", "✗".red(), name, e),
            }
        }

        println!("{} All updates complete!", "✓".green());
        return Ok(());
    };

    let tool = find_tool(&tools, name).with_context(|| {
        format!(
            "Tool '{}' not found. Available tools: {}",
            name,
            format_available_tools(&tools)
        )
    })?;

    if !tool.is_installed() {
        println!(
            "{} {} is not installed. Run `ai-cli apps install {}` first.",
            "!".yellow(),
            tool.name,
            name
        );
        return Ok(());
    }

    println!("Upgrading {}...", tool.name.bright_cyan());
    upgrade_tool(tool.clone()).await?;
    println!("{} {} upgraded successfully!", "✓".green(), tool.name);
    Ok(())
}

async fn install_tool(tool: Tool) -> Result<()> {
    match &tool.install_method {
        InstallMethod::Bootstrap(url) => {
            run_install_script(url, "bootstrap.sh").await?;
        }
        InstallMethod::Amp(url) => {
            run_install_script(url, "amp_install.sh").await?;
        }
        InstallMethod::Npm(package) => {
            let output = Command::new("npm")
                .args(["install", "-g", package])
                .output()
                .context("Failed to run npm install")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("npm install failed for {}: {}", tool.name, stderr.trim());
            }
        }
    }

    Ok(())
}

async fn uninstall_tool(tool: &Tool, remove_config: bool, force: bool) -> Result<()> {
    println!("Uninstalling {}...", tool.name.bright_cyan());

    match &tool.install_method {
        InstallMethod::Bootstrap(_) => {
            let home = std::env::var("HOME").context("HOME environment variable not set")?;
            let binary_name = tool.binary_name.as_deref().unwrap_or(tool.name.as_str());

            let symlink_path = Path::new(&home)
                .join(".local")
                .join("bin")
                .join(binary_name);

            let versions_path = Path::new(&home)
                .join(".local")
                .join("share")
                .join(binary_name)
                .join("versions");
            let config_dirs: Vec<_> = if tool.config_dirs.is_empty() {
                vec![Path::new(&home).join(format!(".{}", binary_name))]
            } else {
                tool.config_dirs
                    .iter()
                    .map(|dir| Path::new(&home).join(dir))
                    .collect()
            };
            let mut existing_configs: Vec<_> = config_dirs
                .into_iter()
                .filter(|path| path.exists())
                .collect();

            let mut removed_items = Vec::new();
            let mut binary_paths = vec![symlink_path];
            binary_paths.extend(
                tool.extra_binary_paths
                    .iter()
                    .map(|extra| Path::new(&home).join(extra)),
            );

            for binary_path in binary_paths {
                if binary_path.exists() {
                    fs::remove_file(&binary_path).with_context(|| {
                        format!("Failed to remove binary {}", binary_path.display())
                    })?;
                    removed_items.push(format!("binary: {}", binary_path.display()));
                }
            }

            if versions_path.exists()
                && let Some(parent) = versions_path.parent()
            {
                fs::remove_dir_all(parent).context("Failed to remove versions directory")?;
                removed_items.push(format!("versions: {}", parent.display()));
            }

            if !existing_configs.is_empty() {
                if existing_configs.len() == 1 {
                    println!(
                        "{} Config directory found at: {}",
                        "→".cyan(),
                        existing_configs[0].display()
                    );
                } else {
                    println!("{} Config directories found:", "→".cyan());
                    for path in &existing_configs {
                        println!("  - {}", path.display());
                    }
                }

                if remove_config {
                    let should_remove = if force {
                        true
                    } else {
                        println!(
                            "{} Remove config directories? (contains settings and history) [y/N]",
                            "?".yellow()
                        );
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        input.trim().eq_ignore_ascii_case("y")
                    };

                    if should_remove {
                        for path in existing_configs.drain(..) {
                            fs::remove_dir_all(&path).with_context(|| {
                                format!("Failed to remove config directory {}", path.display())
                            })?;
                            removed_items.push(format!("config: {}", path.display()));
                        }
                    } else {
                        println!("{} Keeping config directories", "→".cyan());
                    }
                } else {
                    let suffix = if existing_configs.len() > 1 {
                        "directories"
                    } else {
                        "directory"
                    };
                    println!(
                        "{} Keeping config {} (use --remove-config to remove it)",
                        "→".cyan(),
                        suffix
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
            let home_path = Path::new(&home);
            let amp_home = home_path.join(".amp");
            let local_bin = home_path.join(".local").join("bin");
            let mut removed_items = Vec::new();

            for shim in ["amp", "amp.bat"] {
                let shim_path = local_bin.join(shim);
                if shim_path.exists() {
                    fs::remove_file(&shim_path)
                        .with_context(|| format!("Failed to remove {}", shim_path.display()))?;
                    removed_items.push(format!("shim: {}", shim_path.display()));
                }
            }

            if amp_home.exists() {
                fs::remove_dir_all(&amp_home).context("Failed to remove AMP_HOME directory")?;
                removed_items.push(format!("AMP_HOME: {}", amp_home.display()));
            }

            let config_home = std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".config"));
            let data_home = std::env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_path.join(".local").join("share"));
            let cache_home = std::env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
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
                    io::stdin().read_line(&mut input)?;
                    input.trim().eq_ignore_ascii_case("y")
                };

                if should_remove {
                    for path in [
                        config_home.join("amp"),
                        data_home.join("amp"),
                        cache_home.join("amp"),
                    ] {
                        if path.exists() {
                            let metadata = fs::metadata(&path)?;
                            if metadata.is_file() {
                                fs::remove_file(&path).with_context(|| {
                                    format!("Failed to remove {}", path.display())
                                })?;
                            } else {
                                fs::remove_dir_all(&path).with_context(|| {
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
                .args(["uninstall", "-g", package])
                .status()
                .context("Failed to run npm uninstall")?;

            if status.success() {
                println!("{} {} uninstalled successfully!", "✓".green(), tool.name);
            } else {
                anyhow::bail!("npm uninstall failed for {}", tool.name);
            }
        }
    }

    Ok(())
}

async fn upgrade_tool(tool: Tool) -> Result<()> {
    match &tool.install_method {
        InstallMethod::Amp(_) => {
            let output = Command::new("amp")
                .arg("update")
                .output()
                .context("Failed to run `amp update`")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("`amp update` failed: {}", stderr.trim());
            }
        }
        InstallMethod::Npm(package) => {
            let output = Command::new("npm")
                .args(["install", "-g"])
                .arg(package)
                .output()
                .context("Failed to run npm install")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("npm install failed for {}: {}", tool.name, stderr.trim());
            }
        }
        InstallMethod::Bootstrap(url) => {
            let binary_name = tool.binary_name.as_deref().unwrap_or("");

            match binary_name {
                "claude" => {
                    let output = Command::new("claude")
                        .arg("update")
                        .output()
                        .context("Failed to run claude update")?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        anyhow::bail!("claude update failed: {}", stderr.trim());
                    }
                }
                "cursor-agent" => {
                    let output = Command::new("cursor-agent")
                        .arg("upgrade")
                        .output()
                        .context("Failed to run cursor-agent upgrade")?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        anyhow::bail!("cursor-agent upgrade failed: {}", stderr.trim());
                    }
                }
                "opencode" => {
                    let output = Command::new("opencode")
                        .arg("upgrade")
                        .output()
                        .context("Failed to run opencode upgrade")?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        anyhow::bail!("opencode upgrade failed: {}", stderr.trim());
                    }
                }
                _ => {
                    run_install_script(url, "bootstrap_upgrade.sh").await?;
                }
            }
        }
    }

    Ok(())
}

async fn run_install_script(url: &str, temp_filename: &str) -> Result<()> {
    let script = reqwest::get(url)
        .await
        .context("Failed to download install script")?
        .text()
        .await
        .context("Failed to read install script")?;

    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join(temp_filename);
    fs::write(&script_path, &script).context("Failed to write install script")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    let output = Command::new("bash")
        .arg(&script_path)
        .output()
        .context("Failed to run install script")?;

    let _ = fs::remove_file(&script_path);

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Install script failed: {}", stderr.trim());
    }
}

fn format_available_tools(tools: &[Tool]) -> String {
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
}

fn find_tool<'a>(tools: &'a [Tool], name: &str) -> Option<&'a Tool> {
    tools.iter().find(|t| {
        t.name.eq_ignore_ascii_case(name)
            || t.binary_name
                .as_ref()
                .map(|b| b.eq_ignore_ascii_case(name))
                .unwrap_or(false)
    })
}
