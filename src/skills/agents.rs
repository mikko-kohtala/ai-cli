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
    /// Local skills directory (relative path, e.g., ".claude/skills/")
    pub local_skills_dir: &'static str,
    /// Global skills directory path
    pub global_skills_path: PathBuf,
}

impl SkillAgent {
    /// Check if this agent is installed
    pub fn is_installed(&self) -> bool {
        // Special handling for agents without CLI binaries (IDE-based agents)
        let ide_agents = ["cursor", "windsurf", "kilo", "roo", "trae"];
        if ide_agents.contains(&self.binary_name) {
            return self.global_skills_path.parent().is_some_and(|p| p.exists());
        }

        Command::new("which")
            .arg(self.binary_name)
            .output()
            .is_ok_and(|o| o.status.success())
    }

    /// Get local skills path relative to current directory
    pub fn local_skills_path(&self) -> PathBuf {
        std::env::current_dir()
            .unwrap_or_default()
            .join(self.local_skills_dir)
    }

    /// Ensure global skills directory exists
    pub fn ensure_skills_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.global_skills_path)
    }
}

fn home_dir() -> PathBuf {
    dirs::home_dir().expect("Could not find home directory")
}

fn amp() -> SkillAgent {
    SkillAgent {
        name: "Amp",
        id: "amp",
        binary_name: "amp",
        local_skills_dir: ".agents/skills/",
        global_skills_path: home_dir().join(".config/agents/skills"),
    }
}

fn antigravity() -> SkillAgent {
    SkillAgent {
        name: "Antigravity",
        id: "antigravity",
        binary_name: "antigravity",
        local_skills_dir: ".agent/skills/",
        global_skills_path: home_dir().join(".gemini/antigravity/skills"),
    }
}

fn claude_code() -> SkillAgent {
    SkillAgent {
        name: "Claude Code",
        id: "claude",
        binary_name: "claude",
        local_skills_dir: ".claude/skills/",
        global_skills_path: home_dir().join(".claude/skills"),
    }
}

fn clawdbot() -> SkillAgent {
    SkillAgent {
        name: "Clawdbot",
        id: "clawdbot",
        binary_name: "clawdbot",
        local_skills_dir: "skills/",
        global_skills_path: home_dir().join(".clawdbot/skills"),
    }
}

fn codex_cli() -> SkillAgent {
    SkillAgent {
        name: "Codex",
        id: "codex",
        binary_name: "codex",
        local_skills_dir: ".codex/skills/",
        global_skills_path: home_dir().join(".codex/skills"),
    }
}

fn cursor() -> SkillAgent {
    SkillAgent {
        name: "Cursor",
        id: "cursor",
        binary_name: "cursor",
        local_skills_dir: ".cursor/skills/",
        global_skills_path: home_dir().join(".cursor/skills"),
    }
}

fn droid() -> SkillAgent {
    SkillAgent {
        name: "Droid",
        id: "droid",
        binary_name: "droid",
        local_skills_dir: ".factory/skills/",
        global_skills_path: home_dir().join(".factory/skills"),
    }
}

fn gemini_cli() -> SkillAgent {
    SkillAgent {
        name: "Gemini CLI",
        id: "gemini",
        binary_name: "gemini",
        local_skills_dir: ".gemini/skills/",
        global_skills_path: home_dir().join(".gemini/skills"),
    }
}

fn github_copilot() -> SkillAgent {
    SkillAgent {
        name: "GitHub Copilot",
        id: "github-copilot",
        binary_name: "github-copilot",
        local_skills_dir: ".github/skills/",
        global_skills_path: home_dir().join(".copilot/skills"),
    }
}

fn goose() -> SkillAgent {
    SkillAgent {
        name: "Goose",
        id: "goose",
        binary_name: "goose",
        local_skills_dir: ".goose/skills/",
        global_skills_path: home_dir().join(".config/goose/skills"),
    }
}

fn kilo_code() -> SkillAgent {
    SkillAgent {
        name: "Kilo Code",
        id: "kilo",
        binary_name: "kilo",
        local_skills_dir: ".kilocode/skills/",
        global_skills_path: home_dir().join(".kilocode/skills"),
    }
}

fn kiro_cli() -> SkillAgent {
    SkillAgent {
        name: "Kiro CLI",
        id: "kiro",
        binary_name: "kiro",
        local_skills_dir: ".kiro/skills/",
        global_skills_path: home_dir().join(".kiro/skills"),
    }
}

fn opencode() -> SkillAgent {
    SkillAgent {
        name: "OpenCode",
        id: "opencode",
        binary_name: "opencode",
        local_skills_dir: ".opencode/skills/",
        global_skills_path: home_dir().join(".config/opencode/skills"),
    }
}

fn roo_code() -> SkillAgent {
    SkillAgent {
        name: "Roo Code",
        id: "roo",
        binary_name: "roo",
        local_skills_dir: ".roo/skills/",
        global_skills_path: home_dir().join(".roo/skills"),
    }
}

fn trae() -> SkillAgent {
    SkillAgent {
        name: "Trae",
        id: "trae",
        binary_name: "trae",
        local_skills_dir: ".trae/skills/",
        global_skills_path: home_dir().join(".trae/skills"),
    }
}

fn windsurf() -> SkillAgent {
    SkillAgent {
        name: "Windsurf",
        id: "windsurf",
        binary_name: "windsurf",
        local_skills_dir: ".windsurf/skills/",
        global_skills_path: home_dir().join(".codeium/windsurf/skills"),
    }
}

/// Returns all supported AI agents for skills
pub fn catalog() -> Vec<SkillAgent> {
    vec![
        amp(),
        antigravity(),
        claude_code(),
        clawdbot(),
        codex_cli(),
        cursor(),
        droid(),
        gemini_cli(),
        github_copilot(),
        goose(),
        kilo_code(),
        kiro_cli(),
        opencode(),
        roo_code(),
        trae(),
        windsurf(),
    ]
}

/// Find an agent by ID
pub fn find(id: &str) -> Option<SkillAgent> {
    catalog()
        .into_iter()
        .find(|a| a.id.eq_ignore_ascii_case(id))
}
