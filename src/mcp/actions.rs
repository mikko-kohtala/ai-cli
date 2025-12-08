use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{Context, Result};
use colored::Colorize;

use super::servers::{self, McpServer};
use super::targets::{self, McpTarget};

#[derive(Clone, Debug)]
enum ServerStatus {
    Enabled,
    Disabled,
    Unknown,
    NotInstalled,
}

pub fn handle_list() -> Result<()> {
    let servers = servers::catalog();
    let targets = targets::catalog();

    println!("{}", "Available Servers:".bold());
    for server in &servers {
        println!("  {}  {}", server.id.cyan(), server.description.dimmed());
    }
    println!();

    // Check status in parallel
    let statuses = check_statuses_parallel(&targets, &servers);

    // Status table
    println!("{}", "Status per tool:".bold());
    println!();

    // Header
    print!("  {:<16}", "Tool".dimmed());
    for server in &servers {
        print!("  {:<12}", server.id.dimmed());
    }
    println!();

    // Separator
    print!("  {}", "-".repeat(16).dimmed());
    for _ in &servers {
        print!("  {}", "-".repeat(12).dimmed());
    }
    println!();

    // Status rows
    for target in &targets {
        print!("  {:<16}", target.name);

        for server in &servers {
            let key = (target.name, server.id);
            let status = statuses.get(&key).cloned().unwrap_or(ServerStatus::Unknown);
            let status_str = match status {
                ServerStatus::Enabled => format!("{:<12}", "enabled").green().to_string(),
                ServerStatus::Disabled => format!("{:<12}", "disabled").yellow().to_string(),
                ServerStatus::NotInstalled => {
                    format!("{:<12}", "not installed").dimmed().to_string()
                }
                ServerStatus::Unknown => format!("{:<12}", "unknown").dimmed().to_string(),
            };
            print!("  {}", status_str);
        }
        println!();
    }

    Ok(())
}

fn check_statuses_parallel(
    targets: &[McpTarget],
    servers: &[McpServer],
) -> HashMap<(&'static str, &'static str), ServerStatus> {
    let results: Arc<Mutex<HashMap<(&'static str, &'static str), ServerStatus>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for target in targets {
        let target = target.clone();
        let servers: Vec<_> = servers.to_vec();
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let is_installed = target.is_installed();

            for server in servers {
                let status = if !is_installed {
                    ServerStatus::NotInstalled
                } else {
                    match target.is_server_enabled(&server) {
                        Ok(true) => ServerStatus::Enabled,
                        Ok(false) => ServerStatus::Disabled,
                        Err(_) => ServerStatus::Unknown,
                    }
                };

                let mut map = results.lock().unwrap();
                map.insert((target.name, server.id), status);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

pub fn handle_enable(server_name: &str) -> Result<()> {
    let servers_to_enable = if server_name == "all" {
        servers::catalog()
    } else {
        vec![
            servers::find(server_name)
                .with_context(|| format!("Unknown server: {}", server_name))?,
        ]
    };
    let targets = targets::catalog();

    let label = if server_name == "all" {
        "all servers".to_string()
    } else {
        server_name.to_string()
    };

    println!(
        "{}",
        format!("Enabling {} across installed tools...", label).bold()
    );
    println!();

    let mut success_count = 0;
    let mut skip_count = 0;

    for target in &targets {
        print!("  {:<16}", target.name);

        if !target.is_installed() {
            println!("{}", "[SKIP] Not installed".dimmed());
            skip_count += 1;
            continue;
        }

        let mut target_ok = true;
        for server in &servers_to_enable {
            match target.enable_server(server) {
                Ok(_) => {}
                Err(e) => {
                    if target_ok {
                        println!("{} {}", "[FAIL]".red(), e);
                        target_ok = false;
                    }
                }
            }
        }
        if target_ok {
            println!("{}", "[OK]".green());
            success_count += 1;
        }
    }

    println!();
    println!(
        "{}",
        format!(
            "Done! Enabled {} in {} tool(s), skipped {}.",
            label, success_count, skip_count
        )
        .green()
    );
    println!();
    println!(
        "{}",
        "Note: You may need to restart your CLI tools for changes to take effect.".dimmed()
    );

    Ok(())
}

pub fn handle_disable(server_name: &str) -> Result<()> {
    let servers_to_disable = if server_name == "all" {
        servers::catalog()
    } else {
        vec![
            servers::find(server_name)
                .with_context(|| format!("Unknown server: {}", server_name))?,
        ]
    };
    let targets = targets::catalog();

    let label = if server_name == "all" {
        "all servers".to_string()
    } else {
        server_name.to_string()
    };

    println!(
        "{}",
        format!("Disabling {} across installed tools...", label).bold()
    );
    println!();

    let mut success_count = 0;
    let mut skip_count = 0;

    for target in &targets {
        print!("  {:<16}", target.name);

        if !target.is_installed() {
            println!("{}", "[SKIP] Not installed".dimmed());
            skip_count += 1;
            continue;
        }

        let mut target_ok = true;
        for server in &servers_to_disable {
            match target.disable_server(server) {
                Ok(_) => {}
                Err(e) => {
                    if target_ok {
                        println!("{} {}", "[FAIL]".red(), e);
                        target_ok = false;
                    }
                }
            }
        }
        if target_ok {
            println!("{}", "[OK]".green());
            success_count += 1;
        }
    }

    println!();
    println!(
        "{}",
        format!(
            "Done! Disabled {} in {} tool(s), skipped {}.",
            label, success_count, skip_count
        )
        .green()
    );
    println!();
    println!(
        "{}",
        "Note: You may need to restart your CLI tools for changes to take effect.".dimmed()
    );

    Ok(())
}

pub fn handle_doctor() -> Result<()> {
    let targets = targets::catalog();

    for target in &targets {
        let installed = target.is_installed();
        let status = if installed {
            "installed".green()
        } else {
            "not installed".yellow()
        };

        println!("{:<16} [{}]", target.name.bold(), status);
        println!("  {}", target.config_path().display().to_string().dimmed());

        if installed {
            let exists = target.config_path().exists();
            if exists {
                println!("  {}", "config exists".dimmed());
            } else {
                println!("  {}", "config not created yet".dimmed());
            }
        }
        println!();
    }

    Ok(())
}
