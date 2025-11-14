mod actions;
mod cli;
mod tools;
mod versions;

use actions::{handle_install_command, handle_uninstall_command, handle_upgrade_command};
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use tools::installed_versions;
use versions::{check_latest_versions, print_version};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("\n{}", "🤖 AI Tools Manager".bright_cyan().bold());
    println!("{}\n", "=".repeat(19).bright_cyan());

    match cli.command {
        None | Some(Commands::List) => {
            let mut tools = installed_versions();
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
        Some(Commands::Check) => {
            let mut tools = installed_versions();
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
