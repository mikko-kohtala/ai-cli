/// Represents an MCP server that can be enabled/disabled
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct McpServer {
    /// Internal identifier (lowercase, used in CLI)
    pub id: &'static str,
    /// Display name
    pub name: &'static str,
    /// Arguments for npx command
    pub args: &'static [&'static str],
    /// Description for help text
    pub description: &'static str,
}

impl McpServer {
    pub const fn new(
        id: &'static str,
        name: &'static str,
        args: &'static [&'static str],
        description: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            args,
            description,
        }
    }
}

// Server definitions

fn linear() -> McpServer {
    McpServer::new(
        "linear",
        "Linear",
        &["mcp-remote", "https://mcp.linear.app/mcp"],
        "Linear issue tracking integration",
    )
}

fn playwright() -> McpServer {
    McpServer::new(
        "playwright",
        "Playwright",
        &["@playwright/mcp@latest"],
        "Browser automation with Playwright",
    )
}

/// Returns all available MCP servers
pub fn catalog() -> Vec<McpServer> {
    vec![linear(), playwright()]
}

/// Find a server by its ID
pub fn find(id: &str) -> Option<McpServer> {
    catalog().into_iter().find(|s| s.id == id)
}
