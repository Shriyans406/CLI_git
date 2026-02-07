    use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "gitx",
    version = "0.1.0",
    about = "A beginner-friendly Git CLI assistant"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show repository status
    St,

    /// List branches
    Br,

    /// Create and switch to a new branch
    New {
        branch: String,
    },

    /// Guided commit assistant
    Commit,

    /// Clean merged branches
    Cleanup,

    /// Analyze commit history
    Analyze,
}

