mod amp;
mod claude;
mod cline;
mod codex;
mod copilot;
mod cursor_agent;
mod factory;
mod gemini;
mod kilo;
mod opencode;

use std::process::Command;

pub use amp::{definition as amp_tool, installed_version as amp_installed_version};
pub use claude::{definition as claude_tool, installed_version as claude_installed_version};
pub use cline::{definition as cline_tool, installed_version as cline_installed_version};
pub use codex::{definition as codex_tool, installed_version as codex_installed_version};
pub use copilot::{definition as copilot_tool, installed_version as copilot_installed_version};
pub use cursor_agent::{
    definition as cursor_agent_tool, installed_version as cursor_agent_installed_version,
};
pub use factory::{
    definition as factory_cli_tool, installed_version as factory_cli_installed_version,
};
pub use gemini::{definition as gemini_tool, installed_version as gemini_installed_version};
pub use kilo::{definition as kilo_tool, installed_version as kilo_installed_version};
pub use opencode::{definition as opencode_tool, installed_version as opencode_installed_version};

#[derive(Debug, Clone)]
pub enum InstallMethod {
    Npm(String),
    Bootstrap(String),
    Amp(String),
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub install_method: InstallMethod,
    pub check_command: Vec<String>,
    pub binary_name: Option<String>,
    pub config_dirs: Vec<String>,
    pub extra_binary_paths: Vec<String>,
}

impl Tool {
    pub fn new(name: &str, install_method: InstallMethod, check_command: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            install_method,
            check_command,
            binary_name: None,
            config_dirs: Vec::new(),
            extra_binary_paths: Vec::new(),
        }
    }

    pub fn with_binary_name(mut self, binary_name: &str) -> Self {
        self.binary_name = Some(binary_name.to_string());
        self
    }

    pub fn with_config_dir(mut self, config_dir: &str) -> Self {
        self.config_dirs.push(config_dir.to_string());
        self
    }

    pub fn with_extra_binary_path(mut self, path: &str) -> Self {
        self.extra_binary_paths.push(path.to_string());
        self
    }

    pub fn is_installed(&self) -> bool {
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

#[derive(Debug, Clone)]
pub struct ToolVersion {
    pub name: String,
    pub installed: Option<String>,
    pub latest: Option<String>,
    pub identifier: Option<String>,
}

impl ToolVersion {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            installed: None,
            latest: None,
            identifier: None,
        }
    }

    pub fn with_installed(mut self, version: Option<String>) -> Self {
        self.installed = version;
        self
    }

    pub fn with_identifier(mut self, identifier: &str) -> Self {
        self.identifier = Some(identifier.to_string());
        self
    }
}

pub fn catalog() -> Vec<Tool> {
    vec![
        claude_tool(),
        amp_tool(),
        codex_tool(),
        cursor_agent_tool(),
        copilot_tool(),
        kilo_tool(),
        gemini_tool(),
        cline_tool(),
        opencode_tool(),
        factory_cli_tool(),
    ]
}

pub fn installed_versions() -> Vec<ToolVersion> {
    vec![
        claude_installed_version(),
        amp_installed_version(),
        codex_installed_version(),
        cursor_agent_installed_version(),
        copilot_installed_version(),
        kilo_installed_version(),
        gemini_installed_version(),
        cline_installed_version(),
        opencode_installed_version(),
        factory_cli_installed_version(),
    ]
}

pub(crate) fn command_output(cmd: &str, args: &[&str]) -> Option<String> {
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
