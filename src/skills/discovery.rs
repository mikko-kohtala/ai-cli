use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Represents a skill found in a repository
#[derive(Debug, Clone)]
pub struct Skill {
    /// Skill name from frontmatter
    pub name: String,
    /// Description from frontmatter
    pub description: Option<String>,
    /// Path to the skill directory (containing SKILL.md)
    pub path: PathBuf,
}

/// Discovery priority order for finding SKILL.md files
const DISCOVERY_PATHS: &[&str] = &[
    "",                     // Root directory
    "skills",               // skills/
    "skills/.curated",      // skills/.curated/
    "skills/.experimental", // skills/.experimental/
];

/// Discover skills in a cloned repository
pub fn discover_skills(repo_path: &Path) -> Result<Vec<Skill>> {
    let mut skills = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    // Try priority paths first
    for subpath in DISCOVERY_PATHS {
        let search_path = if subpath.is_empty() {
            repo_path.to_path_buf()
        } else {
            repo_path.join(subpath)
        };

        if !search_path.exists() {
            continue;
        }

        // Check for SKILL.md directly in this path
        try_add_skill(&search_path, &mut skills, &mut seen_names);

        // Check subdirectories for SKILL.md files
        let Ok(entries) = std::fs::read_dir(&search_path) else {
            continue;
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                try_add_skill(&path, &mut skills, &mut seen_names);
            }
        }
    }

    // If no skills found in priority paths, do recursive search (max 5 levels)
    if skills.is_empty() {
        find_skills_recursive(repo_path, 0, 5, &mut skills, &mut seen_names)?;
    }

    Ok(skills)
}

fn try_add_skill(
    dir: &Path,
    skills: &mut Vec<Skill>,
    seen_names: &mut std::collections::HashSet<String>,
) {
    let skill_file = dir.join("SKILL.md");
    let Ok(skill) = parse_skill(&skill_file, dir) else {
        return;
    };
    if seen_names.insert(skill.name.clone()) {
        skills.push(skill);
    }
}

fn find_skills_recursive(
    path: &Path,
    depth: usize,
    max_depth: usize,
    skills: &mut Vec<Skill>,
    seen_names: &mut std::collections::HashSet<String>,
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    try_add_skill(path, skills, seen_names);

    let Ok(entries) = std::fs::read_dir(path) else {
        return Ok(());
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let dominated_by_dot = entry_path
            .file_name()
            .is_some_and(|n| n.to_str().is_some_and(|s| s.starts_with('.')));

        if entry_path.is_dir() && !dominated_by_dot {
            find_skills_recursive(&entry_path, depth + 1, max_depth, skills, seen_names)?;
        }
    }

    Ok(())
}

/// Parse a SKILL.md file and extract frontmatter
fn parse_skill(skill_file: &Path, skill_dir: &Path) -> Result<Skill> {
    if !skill_file.exists() {
        anyhow::bail!("SKILL.md not found");
    }

    let content = std::fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read {}", skill_file.display()))?;

    // Parse YAML frontmatter (between --- markers)
    let (name, description) = parse_frontmatter(&content)?;

    Ok(Skill {
        name,
        description,
        path: skill_dir.to_path_buf(),
    })
}

fn parse_frontmatter(content: &str) -> Result<(String, Option<String>)> {
    let content = content.trim();

    if !content.starts_with("---") {
        anyhow::bail!("SKILL.md must start with YAML frontmatter (---)");
    }

    let rest = &content[3..];
    let end_idx = rest
        .find("---")
        .context("SKILL.md frontmatter not properly closed with ---")?;

    let yaml_content = &rest[..end_idx];

    // Simple YAML parsing for name, description
    let mut name = None;
    let mut description = None;

    for line in yaml_content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("name:") {
            name = Some(
                trimmed
                    .strip_prefix("name:")
                    .unwrap()
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        } else if trimmed.starts_with("description:") {
            description = Some(
                trimmed
                    .strip_prefix("description:")
                    .unwrap()
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }

    let name = name.context("SKILL.md must have a 'name' field in frontmatter")?;

    Ok((name, description))
}

/// List installed skills for an agent
pub fn list_installed_skills(skills_path: &Path) -> Result<Vec<Skill>> {
    let mut skills = Vec::new();

    if !skills_path.exists() {
        return Ok(skills);
    }

    let Ok(entries) = std::fs::read_dir(skills_path) else {
        return Ok(skills);
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let skill_file = path.join("SKILL.md");
            if let Ok(skill) = parse_skill(&skill_file, &path) {
                skills.push(skill);
            }
        }
    }

    Ok(skills)
}
