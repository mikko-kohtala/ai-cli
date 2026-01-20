use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;
use tempfile::TempDir;

use super::agents::{self, SkillAgent};
use super::discovery::{self, Skill};

/// Handle `skills list` command
pub fn handle_list(agent_filter: Option<&str>) -> Result<()> {
    let agents = if let Some(agent_id) = agent_filter {
        vec![agents::find(agent_id).with_context(|| format!("Unknown agent: {}", agent_id))?]
    } else {
        agents::catalog()
    };

    // Collect data for installed and not installed agents
    let mut installed: Vec<(String, Vec<Skill>, Vec<Skill>)> = Vec::new();
    let mut not_installed: Vec<String> = Vec::new();

    for agent in &agents {
        if !agent.is_installed() {
            not_installed.push(agent.name.to_string());
            continue;
        }
        let local = discovery::list_installed_skills(&agent.local_skills_path())?;
        let global = discovery::list_installed_skills(&agent.global_skills_path)?;
        installed.push((agent.name.to_string(), local, global));
    }

    if installed.is_empty() && not_installed.is_empty() {
        println!("{}", "No AI agents found".dimmed());
        return Ok(());
    }

    // Calculate widths for alignment
    let agent_width = installed
        .iter()
        .map(|(n, _, _)| n.len())
        .chain(not_installed.iter().map(|n| n.len()))
        .max()
        .unwrap_or(0)
        + 1; // +1 for colon
    let skill_width = installed
        .iter()
        .flat_map(|(_, local, global)| local.iter().chain(global.iter()))
        .map(|s| s.name.len())
        .max()
        .unwrap_or(0);

    // Print installed agents
    if !installed.is_empty() {
        println!("{}", "Installed:".bright_green().bold());
        for (name, local, global) in &installed {
            if local.is_empty() && global.is_empty() {
                println!(
                    "{:width$} {}",
                    format!("{}:", name),
                    "(no skills)".dimmed(),
                    width = agent_width
                );
            } else {
                for (i, skill) in local.iter().chain(global.iter()).enumerate() {
                    let label = if i == 0 {
                        format!("{}:", name)
                    } else {
                        String::new()
                    };
                    let is_local = i < local.len();
                    print_skill(&label, agent_width, skill, is_local, skill_width);
                }
            }
        }
    }

    // Print not installed agents
    if !not_installed.is_empty() {
        if !installed.is_empty() {
            println!();
        }
        println!("{}", "Not Installed:".bright_black().bold());
        for name in &not_installed {
            println!("{:width$}", format!("{}:", name), width = agent_width);
        }
    }

    Ok(())
}

fn print_skill(label: &str, label_width: usize, skill: &Skill, is_local: bool, skill_width: usize) {
    let desc = skill
        .description
        .as_ref()
        .map(|d| {
            let max = 45;
            if d.len() > max {
                format!("{}...", &d[..max - 3])
            } else {
                d.clone()
            }
        })
        .unwrap_or_default();

    print!("{:label_width$} {:skill_width$}", label, skill.name);

    if is_local {
        print!("  {}", "(local)".dimmed());
    } else if !desc.is_empty() {
        print!("  {}", desc.dimmed());
    }
    println!();
}

/// Handle `skills install <repo>` command
pub fn handle_install(repo: &str, agent_filter: Option<&str>) -> Result<()> {
    // Parse repo input (owner/repo or full URL)
    let repo_url = parse_repo_url(repo)?;

    // Clone to temp directory
    println!("{} Cloning {}...", "->".cyan(), repo);
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;

    let status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            &repo_url,
            temp_dir.path().to_str().unwrap(),
        ])
        .status()
        .context("Failed to run git clone")?;

    if !status.success() {
        anyhow::bail!("git clone failed for {}", repo);
    }

    // Discover skills in repo
    let skills = discovery::discover_skills(temp_dir.path())?;

    if skills.is_empty() {
        anyhow::bail!("No skills found in repository (no SKILL.md files)");
    }

    println!("{} Found {} skill(s):", "->".cyan(), skills.len());
    for skill in &skills {
        println!("  {} {}", "-".cyan(), skill.name);
    }
    println!();

    // Get target agents
    let agents: Vec<SkillAgent> = if let Some(agent_id) = agent_filter {
        vec![agents::find(agent_id).with_context(|| format!("Unknown agent: {}", agent_id))?]
    } else {
        agents::catalog()
            .into_iter()
            .filter(|a| a.is_installed())
            .collect()
    };

    if agents.is_empty() {
        anyhow::bail!("No AI agents installed to install skills to");
    }

    // Install skills to each agent
    println!("{}", "Installing skills:".bold());

    for agent in &agents {
        print!("  {:<16}", agent.name);

        if !agent.is_installed() {
            println!("{}", "[SKIP] Not installed".dimmed());
            continue;
        }

        // Ensure skills directory exists
        agent
            .ensure_skills_dir()
            .with_context(|| format!("Failed to create skills directory for {}", agent.name))?;

        // Copy each skill
        for skill in &skills {
            let dest = agent.global_skills_path.join(&skill.name);

            // Remove existing skill if present
            if dest.exists() {
                std::fs::remove_dir_all(&dest)
                    .with_context(|| format!("Failed to remove existing skill {}", skill.name))?;
            }

            // Copy skill directory
            copy_dir_recursive(&skill.path, &dest)
                .with_context(|| format!("Failed to copy skill {}", skill.name))?;
        }

        println!("{}", "[OK]".green());
    }

    println!();
    println!("{}", "Skills installed successfully!".green());

    Ok(())
}

/// Handle `skills remove <skill>` command
pub fn handle_remove(skill_name: &str, agent_filter: Option<&str>) -> Result<()> {
    let agents = if let Some(agent_id) = agent_filter {
        vec![agents::find(agent_id).with_context(|| format!("Unknown agent: {}", agent_id))?]
    } else {
        agents::catalog()
    };

    println!("{}", format!("Removing skill '{}':", skill_name).bold());

    let mut removed_count = 0;

    for agent in &agents {
        print!("  {:<16}", agent.name);

        if !agent.is_installed() {
            println!("{}", "[SKIP] Not installed".dimmed());
            continue;
        }

        let skill_path = agent.global_skills_path.join(skill_name);

        if !skill_path.exists() {
            println!("{}", "[SKIP] Not found".dimmed());
            continue;
        }

        std::fs::remove_dir_all(&skill_path)
            .with_context(|| format!("Failed to remove skill from {}", agent.name))?;

        println!("{}", "[OK]".green());
        removed_count += 1;
    }

    println!();
    if removed_count == 0 {
        println!(
            "{}",
            format!("Skill '{}' not found in any agent", skill_name).yellow()
        );
    } else {
        println!(
            "{}",
            format!("Removed skill from {} agent(s)", removed_count).green()
        );
    }

    Ok(())
}

/// Parse repository input to full URL
fn parse_repo_url(repo: &str) -> Result<String> {
    if repo.starts_with("https://") || repo.starts_with("git@") {
        Ok(repo.to_string())
    } else if repo.contains('/') {
        // GitHub shorthand: owner/repo
        Ok(format!("https://github.com/{}.git", repo))
    } else {
        anyhow::bail!("Invalid repository format. Use 'owner/repo' or full URL");
    }
}

/// Recursively copy directory contents
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            // Skip .git directory
            if src_path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
