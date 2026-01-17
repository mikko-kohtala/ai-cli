use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;
use tempfile::TempDir;

use super::agents::{self, SkillAgent};
use super::discovery;

/// Handle `skills list` command
pub fn handle_list(agent_filter: Option<&str>) -> Result<()> {
    let agents = if let Some(agent_id) = agent_filter {
        vec![agents::find(agent_id).with_context(|| format!("Unknown agent: {}", agent_id))?]
    } else {
        agents::catalog()
    };

    for agent in &agents {
        println!("{}", agent.name.bold());

        if !agent.is_installed() {
            println!("  {}", "(not installed)".dimmed());
            println!();
            continue;
        }

        let skills = discovery::list_installed_skills(&agent.skills_path)?;

        if skills.is_empty() {
            println!("  {}", "(no skills installed)".dimmed());
        } else {
            for skill in skills {
                print!("  {} {}", "-".cyan(), skill.name);
                if let Some(desc) = &skill.description {
                    // Truncate description if too long
                    let truncated = if desc.len() > 60 {
                        format!("{}...", &desc[..57])
                    } else {
                        desc.clone()
                    };
                    print!(" - {}", truncated.dimmed());
                }
                println!();
            }
        }
        println!();
    }

    Ok(())
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
            let dest = agent.skills_path.join(&skill.name);

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

        let skill_path = agent.skills_path.join(skill_name);

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
