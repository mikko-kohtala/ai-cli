use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-cli")]
#[command(arg_required_else_help = true)]
#[command(disable_version_flag = true)]
#[command(about = "AI CLI tools", version)]
pub struct Cli {
    /// Print version
    #[arg(short = 'v', long, action = clap::ArgAction::Version)]
    version: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage AI CLI tools (install, update, uninstall)
    #[command(arg_required_else_help = false)]
    Apps {
        #[command(subcommand)]
        command: Option<AppsCommands>,
    },
    /// Manage MCP servers across AI CLI tools
    #[command(arg_required_else_help = false)]
    Mcp {
        #[command(subcommand)]
        command: Option<McpCommands>,
    },
}

#[derive(Subcommand)]
pub enum AppsCommands {
    /// Check latest versions available
    Check,
    /// Upgrade AI CLI tools (optionally specify tool name, e.g., 'amp')
    Upgrade {
        /// Optional tool name to upgrade directly (e.g., 'amp')
        tool: Option<String>,
    },
    /// Update AI CLI tools (alias for upgrade)
    Update {
        /// Optional tool name to update directly (e.g., 'amp')
        tool: Option<String>,
    },
    /// Install AI CLI tools (optionally specify tool name, e.g., 'claude')
    Install {
        /// Optional tool name to install directly (e.g., 'claude')
        tool: Option<String>,
    },
    /// Install AI CLI tools (alias for install)
    Add {
        /// Optional tool name to install directly (e.g., 'claude')
        tool: Option<String>,
    },
    /// Uninstall AI CLI tools (optionally specify tool name, e.g., 'claude')
    Uninstall {
        /// Optional tool name to uninstall directly (e.g., 'claude')
        tool: Option<String>,
        /// Remove config directory (will ask for confirmation unless --force is used)
        #[arg(long)]
        remove_config: bool,
        /// Skip all confirmation prompts
        #[arg(long)]
        force: bool,
    },
    /// Uninstall AI CLI tools (alias for uninstall)
    Remove {
        /// Optional tool name to uninstall directly (e.g., 'claude')
        tool: Option<String>,
        /// Remove config directory (will ask for confirmation unless --force is used)
        #[arg(long)]
        remove_config: bool,
        /// Skip all confirmation prompts
        #[arg(long)]
        force: bool,
    },
    /// List installed AI CLI tools (alias for default command)
    List,
}

#[derive(Subcommand)]
pub enum McpCommands {
    /// List MCP servers and their status across tools
    List,
    /// Enable an MCP server across all installed tools
    Enable {
        /// Server to enable (e.g., 'linear', 'playwright', or 'all')
        server: String,
    },
    /// Disable an MCP server across all installed tools
    Disable {
        /// Server to disable (e.g., 'linear', 'playwright', or 'all')
        server: String,
    },
    /// Show installed tools and their config paths
    Doctor,
}
