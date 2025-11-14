use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-cli-apps")]
#[command(arg_required_else_help = false)]
#[command(disable_version_flag = true)]
#[command(
    about = "Check and manage AI CLI tools versions",
    long_about = "Check and manage AI CLI tools versions\n\nSupported tools:\n  Claude Code (claude)\n  Amp (amp)\n  Codex (codex)\n  Cursor (cursor)\n  Copilot CLI (copilot)\n  Kilo (kilo)\n  Gemini (gemini)\n  Cline (cline)",
    version
)]
pub struct Cli {
    /// Print version
    #[arg(short = 'v', long, action = clap::ArgAction::Version)]
    version: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
