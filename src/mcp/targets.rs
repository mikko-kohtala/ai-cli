use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use serde_json::{json, Value};

use super::servers::McpServer;

/// How a CLI tool configures MCP servers
#[derive(Debug, Clone)]
pub enum ConfigMethod {
    /// JSON config file with mcpServers object
    JsonConfig {
        path: PathBuf,
        /// Key path like "mcpServers" or "amp.mcpServers"
        servers_key: &'static str,
        /// Server name override (e.g., "Playwright" instead of "playwright")
        server_name_override: Option<&'static str>,
        /// Type field value: None, Some("stdio"), or Some("local")
        type_value: Option<&'static str>,
        /// Include "tools": ["*"] field (Copilot format)
        include_tools_field: bool,
    },
    /// TOML config file with [mcp_servers.<name>] sections
    TomlConfig { path: PathBuf },
}

/// Represents a target CLI tool that supports MCP servers
#[derive(Debug, Clone)]
pub struct McpTarget {
    pub name: &'static str,
    pub binary_name: &'static str,
    pub config_method: ConfigMethod,
}

impl McpTarget {
    /// Get the config file path for this target
    pub fn config_path(&self) -> &std::path::Path {
        match &self.config_method {
            ConfigMethod::JsonConfig { path, .. } => path,
            ConfigMethod::TomlConfig { path } => path,
        }
    }

    /// Check if this CLI tool is installed
    pub fn is_installed(&self) -> bool {
        match &self.config_method {
            ConfigMethod::JsonConfig { path, .. } => {
                // For tools like Cursor that may not have a CLI binary,
                // check if their config directory exists
                if self.binary_name == "cursor" {
                    path.parent().is_some_and(|p| p.exists())
                } else if self.binary_name == "copilot" {
                    // Copilot: check binary OR config dir exists
                    Command::new("which")
                        .arg(self.binary_name)
                        .output()
                        .is_ok_and(|o| o.status.success())
                        || path.parent().is_some_and(|p| p.exists())
                } else {
                    Command::new("which")
                        .arg(self.binary_name)
                        .output()
                        .is_ok_and(|o| o.status.success())
                }
            }
            ConfigMethod::TomlConfig { path } => {
                // Check if the tool binary exists or if config exists
                Command::new("which")
                    .arg(self.binary_name)
                    .output()
                    .is_ok_and(|o| o.status.success())
                    || path.exists()
            }
        }
    }

    /// Enable an MCP server for this target
    pub fn enable_server(&self, server: &McpServer) -> Result<String> {
        match &self.config_method {
            ConfigMethod::JsonConfig {
                path,
                servers_key,
                server_name_override,
                type_value,
                include_tools_field,
            } => {
                let server_name = server_name_override.unwrap_or(server.id);
                enable_in_json(
                    path,
                    servers_key,
                    server_name,
                    server,
                    *type_value,
                    *include_tools_field,
                )?;
                Ok(format!("Updated {}", path.display()))
            }
            ConfigMethod::TomlConfig { path } => {
                enable_in_toml(path, server)?;
                Ok(format!("Updated {}", path.display()))
            }
        }
    }

    /// Disable an MCP server for this target
    pub fn disable_server(&self, server: &McpServer) -> Result<String> {
        match &self.config_method {
            ConfigMethod::JsonConfig {
                path,
                servers_key,
                server_name_override,
                ..
            } => {
                let server_name = server_name_override.unwrap_or(server.id);
                disable_in_json(path, servers_key, server_name)?;
                Ok(format!("Updated {}", path.display()))
            }
            ConfigMethod::TomlConfig { path } => {
                disable_in_toml(path, server)?;
                Ok(format!("Updated {}", path.display()))
            }
        }
    }

    /// Check if an MCP server is currently enabled
    pub fn is_server_enabled(&self, server: &McpServer) -> Result<bool> {
        match &self.config_method {
            ConfigMethod::JsonConfig {
                path,
                servers_key,
                server_name_override,
                ..
            } => {
                let server_name = server_name_override.unwrap_or(server.id);
                is_enabled_in_json(path, servers_key, server_name)
            }
            ConfigMethod::TomlConfig { path } => is_enabled_in_toml(path, server),
        }
    }
}

// Target definitions

fn claude_code() -> McpTarget {
    McpTarget {
        name: "Claude Code",
        binary_name: "claude",
        config_method: ConfigMethod::JsonConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".claude.json"),
            servers_key: "mcpServers",
            server_name_override: None,
            type_value: Some("stdio"),
            include_tools_field: false,
        },
    }
}

fn gemini_cli() -> McpTarget {
    McpTarget {
        name: "Gemini CLI",
        binary_name: "gemini",
        config_method: ConfigMethod::JsonConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".gemini/settings.json"),
            servers_key: "mcpServers",
            server_name_override: None,
            type_value: None,
            include_tools_field: false,
        },
    }
}

fn codex_cli() -> McpTarget {
    McpTarget {
        name: "Codex CLI",
        binary_name: "codex",
        config_method: ConfigMethod::TomlConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".codex/config.toml"),
        },
    }
}

fn amp() -> McpTarget {
    McpTarget {
        name: "Amp",
        binary_name: "amp",
        config_method: ConfigMethod::JsonConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".config/amp/settings.json"),
            servers_key: "amp.mcpServers",
            server_name_override: None,
            type_value: None,
            include_tools_field: false,
        },
    }
}

fn cursor() -> McpTarget {
    McpTarget {
        name: "Cursor",
        binary_name: "cursor",
        config_method: ConfigMethod::JsonConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".cursor/mcp.json"),
            servers_key: "mcpServers",
            server_name_override: None,
            type_value: None,
            include_tools_field: false,
        },
    }
}

fn copilot_cli() -> McpTarget {
    McpTarget {
        name: "Copilot CLI",
        binary_name: "copilot",
        config_method: ConfigMethod::JsonConfig {
            path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".copilot/mcp-config.json"),
            servers_key: "mcpServers",
            server_name_override: None,
            type_value: Some("local"),
            include_tools_field: true,
        },
    }
}

/// Returns all supported CLI tools that can have MCP servers configured
pub fn catalog() -> Vec<McpTarget> {
    vec![
        claude_code(),
        gemini_cli(),
        codex_cli(),
        amp(),
        cursor(),
        copilot_cli(),
    ]
}

// JSON config helpers

fn navigate_to_key<'a>(config: &'a Value, key: &str) -> Option<&'a Value> {
    config.get(key)
}

fn navigate_or_create<'a>(config: &'a mut Value, key: &str) -> &'a mut Value {
    if !config.get(key).is_some_and(|v| v.is_object()) {
        config[key] = json!({});
    }
    &mut config[key]
}

fn enable_in_json(
    path: &PathBuf,
    servers_key: &str,
    server_name: &str,
    server: &McpServer,
    type_value: Option<&str>,
    include_tools_field: bool,
) -> Result<()> {
    let mut config: Value = if path.exists() {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON in {}", path.display()))?
    } else {
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
        json!({})
    };

    let servers_obj = navigate_or_create(&mut config, servers_key);
    let mut server_config = json!({
        "command": "npx",
        "args": server.args
    });

    if let Some(type_val) = type_value {
        server_config["type"] = json!(type_val);
        if type_val == "stdio" {
            server_config["env"] = json!({});
        }
    }

    if include_tools_field {
        server_config["tools"] = json!(["*"]);
    }

    servers_obj[server_name] = server_config;

    let content = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn disable_in_json(path: &PathBuf, servers_key: &str, server_name: &str) -> Result<()> {
    if !path.exists() {
        return Ok(()); // Nothing to disable
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let mut config: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

    // Navigate to servers object and remove the server
    if let Some(servers) = config.get_mut(servers_key).and_then(|v| v.as_object_mut()) {
        servers.remove(server_name);
    }

    let content = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn is_enabled_in_json(path: &PathBuf, servers_key: &str, server_name: &str) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let config: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

    let servers = navigate_to_key(&config, servers_key);
    Ok(servers.is_some_and(|s| s.get(server_name).is_some()))
}

// TOML config helpers

fn enable_in_toml(path: &PathBuf, server: &McpServer) -> Result<()> {
    use toml_edit::{value, Array, DocumentMut};

    let mut doc: DocumentMut = if path.exists() {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        content
            .parse()
            .with_context(|| format!("Failed to parse TOML in {}", path.display()))?
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
        DocumentMut::new()
    };

    // Ensure [mcp_servers] table exists
    if !doc.contains_key("mcp_servers") {
        doc["mcp_servers"] = toml_edit::table();
    }

    // Add [mcp_servers.<server_id>]
    let mcp_servers = doc["mcp_servers"].as_table_mut().unwrap();
    if !mcp_servers.contains_key(server.id) {
        mcp_servers[server.id] = toml_edit::table();
    }

    let server_table = mcp_servers[server.id].as_table_mut().unwrap();
    server_table["command"] = value("npx");

    let mut args = Array::new();
    for arg in server.args {
        args.push(*arg);
    }
    server_table["args"] = value(args);

    std::fs::write(path, doc.to_string())
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn disable_in_toml(path: &PathBuf, server: &McpServer) -> Result<()> {
    use toml_edit::DocumentMut;

    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let mut doc: DocumentMut = content
        .parse()
        .with_context(|| format!("Failed to parse TOML in {}", path.display()))?;

    if let Some(mcp_servers) = doc.get_mut("mcp_servers").and_then(|t| t.as_table_mut()) {
        mcp_servers.remove(server.id);
    }

    std::fs::write(path, doc.to_string())
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn is_enabled_in_toml(path: &PathBuf, server: &McpServer) -> Result<bool> {
    use toml_edit::DocumentMut;

    if !path.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let doc: DocumentMut = content
        .parse()
        .with_context(|| format!("Failed to parse TOML in {}", path.display()))?;

    Ok(doc
        .get("mcp_servers")
        .and_then(|t| t.as_table())
        .is_some_and(|t| t.contains_key(server.id)))
}
