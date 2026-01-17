use std::path::PathBuf;
use std::process::Command;

/// Represents an AI agent that can have skills installed
#[derive(Debug, Clone)]
pub struct SkillAgent {
    /// Display name
    pub name: &'static str,
    /// CLI identifier (used with --agent flag)
    pub id: &'static str,
    /// Binary name to check if installed
    pub binary_name: &'static str,
    /// Global skills directory path
    pub skills_path: PathBuf,
}

impl SkillAgent {
    /// Check if this agent is installed
    pub fn is_installed(&self) -> bool {
        // Special handling for agents without CLI binaries (like Cursor)
        if self.binary_name == "cursor" {
            return self.skills_path.parent().is_some_and(|p| p.exists());
        }

        Command::new("which")
            .arg(self.binary_name)
            .output()
            .is_ok_and(|o| o.status.success())
    }

    /// Ensure skills directory exists
    pub fn ensure_skills_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.skills_path)
    }
}

fn home_dir() -> PathBuf {
    dirs::home_dir().expect("Could not find home directory")
}

fn claude_code() -> SkillAgent {
    SkillAgent {
        name: "Claude Code",
        id: "claude",
        binary_name: "claude",
        skills_path: home_dir().join(".claude/skills"),
    }
}

fn gemini_cli() -> SkillAgent {
    SkillAgent {
        name: "Gemini CLI",
        id: "gemini",
        binary_name: "gemini",
        skills_path: home_dir().join(".gemini/skills"),
    }
}

fn codex_cli() -> SkillAgent {
    SkillAgent {
        name: "Codex CLI",
        id: "codex",
        binary_name: "codex",
        skills_path: home_dir().join(".codex/skills"),
    }
}

fn amp() -> SkillAgent {
    SkillAgent {
        name: "Amp",
        id: "amp",
        binary_name: "amp",
        skills_path: home_dir().join(".config/agents/skills"),
    }
}

fn cursor() -> SkillAgent {
    SkillAgent {
        name: "Cursor",
        id: "cursor",
        binary_name: "cursor",
        skills_path: home_dir().join(".cursor/skills"),
    }
}

fn copilot_cli() -> SkillAgent {
    SkillAgent {
        name: "GitHub Copilot",
        id: "copilot",
        binary_name: "copilot",
        skills_path: home_dir().join(".copilot/skills"),
    }
}

fn opencode() -> SkillAgent {
    SkillAgent {
        name: "OpenCode",
        id: "opencode",
        binary_name: "opencode",
        skills_path: home_dir().join(".config/opencode/skill"),
    }
}

/// Returns all supported AI agents for skills
pub fn catalog() -> Vec<SkillAgent> {
    vec![
        claude_code(),
        gemini_cli(),
        codex_cli(),
        amp(),
        cursor(),
        copilot_cli(),
        opencode(),
    ]
}

/// Find an agent by ID
pub fn find(id: &str) -> Option<SkillAgent> {
    catalog()
        .into_iter()
        .find(|a| a.id.eq_ignore_ascii_case(id))
}
