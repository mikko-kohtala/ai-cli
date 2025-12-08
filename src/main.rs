mod actions;
mod cli;
mod mcp;
mod tools;
mod versions;

use actions::{handle_install_command, handle_uninstall_command, handle_upgrade_command};
use anyhow::Result;
use clap::Parser;
use cli::{AppsCommands, Cli, Commands, McpCommands};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use tools::installed_versions;
use versions::{check_latest_versions, print_version};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Apps { command }) => {
            println!("\n{}", "🤖 AI Tools Manager".bright_cyan().bold());
            println!("{}\n", "=".repeat(19).bright_cyan());

            match command {
                None | Some(AppsCommands::List) => {
                    let spinner = ProgressBar::new_spinner();
                    spinner.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap(),
                    );
                    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
                    spinner.set_message("Checking installed tools...");

                    let mut tools = installed_versions();
                    spinner.finish_and_clear();

                    check_latest_versions(&mut tools).await;

                    let label_width = tools.iter().map(|t| t.name.len()).max().unwrap_or(0);
                    let id_width = tools
                        .iter()
                        .map(|t| t.identifier.as_ref().map(|id| id.len()).unwrap_or(0))
                        .max()
                        .unwrap_or(0);
                    let installed: Vec<_> = tools.iter().filter(|t| t.installed.is_some()).collect();
                    let not_installed: Vec<_> = tools.iter().filter(|t| t.installed.is_none()).collect();

                    let all_up_to_date = installed.iter().all(|t| {
                        if let (Some(installed_ver), Some(latest_ver)) = (&t.installed, &t.latest) {
                            installed_ver.contains(latest_ver) || latest_ver.contains(installed_ver)
                        } else {
                            true
                        }
                    });

                    if !installed.is_empty() {
                        println!("{}", "Installed:".bright_green().bold());
                        for tool in &installed {
                            print_version(tool, true, label_width, id_width);
                        }
                        if all_up_to_date {
                            println!("\n{}", "✓ All tools are up to date".green());
                        }
                    }

                    if !not_installed.is_empty() {
                        if !installed.is_empty() {
                            println!();
                        }
                        println!("{}", "Not Installed:".bright_black().bold());
                        for tool in &not_installed {
                            print_version(tool, true, label_width, id_width);
                        }
                    }
                }
                Some(AppsCommands::Check) => {
                    let spinner = ProgressBar::new_spinner();
                    spinner.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.cyan} {msg}")
                            .unwrap(),
                    );
                    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
                    spinner.set_message("Checking installed tools...");

                    let mut tools = installed_versions();
                    spinner.finish_and_clear();

                    check_latest_versions(&mut tools).await;
                    let label_width = tools.iter().map(|t| t.name.len()).max().unwrap_or(0);
                    let id_width = tools
                        .iter()
                        .map(|t| t.identifier.as_ref().map(|id| id.len()).unwrap_or(0))
                        .max()
                        .unwrap_or(0);
                    println!();
                    for tool in &tools {
                        print_version(tool, true, label_width, id_width);
                    }
                }
                Some(AppsCommands::Upgrade { tool }) | Some(AppsCommands::Update { tool }) => {
                    handle_upgrade_command(tool.as_deref()).await?;
                }
                Some(AppsCommands::Install { tool }) | Some(AppsCommands::Add { tool }) => {
                    handle_install_command(tool.as_deref()).await?;
                }
                Some(AppsCommands::Uninstall {
                    tool,
                    remove_config,
                    force,
                })
                | Some(AppsCommands::Remove {
                    tool,
                    remove_config,
                    force,
                }) => {
                    handle_uninstall_command(tool.as_deref(), remove_config, force).await?;
                }
            }

            println!();
        }
        Some(Commands::Mcp { command }) => {
            println!("\n{}", "🔌 MCP Server Manager".bright_cyan().bold());
            println!("{}\n", "=".repeat(21).bright_cyan());

            match command {
                None | Some(McpCommands::List) => {
                    mcp::handle_list()?;
                }
                Some(McpCommands::Enable { server }) => {
                    mcp::handle_enable(&server)?;
                }
                Some(McpCommands::Disable { server }) => {
                    mcp::handle_disable(&server)?;
                }
                Some(McpCommands::Doctor) => {
                    mcp::handle_doctor()?;
                }
            }

            println!();
        }
        None => {
            // This won't happen due to arg_required_else_help = true
            unreachable!()
        }
    }

    Ok(())
}
