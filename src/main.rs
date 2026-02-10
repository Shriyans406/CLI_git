mod cli;
mod git;
mod error;
mod status;
mod branch;
mod commit;
mod ui;
mod analyze;

use clap::Parser;
use cli::{Cli, Commands};
use git::ensure_git_repo;
use status::show_status;
use branch::{list_branches, create_branch, cleanup_branches};
use commit::run_commit;
use analyze::run as run_analyze;   // ← ADD THIS

fn main() {
    let cli = Cli::parse();

    // Ensure we are inside a git repository
    if let Err(e) = ensure_git_repo() {
        eprintln!("{}", e);
        return;
    }

    match cli.command {
        Commands::St => {
            if let Err(e) = show_status() {
                eprintln!("{}", e);
            }
        }

        Commands::Br => {
            if let Err(e) = list_branches() {
                eprintln!("{}", e);
            }
        }

        Commands::New { branch } => {
            if let Err(e) = create_branch(&branch) {
                eprintln!("{}", e);
            }
        }

        Commands::Commit => {
            if let Err(e) = run_commit() {
                eprintln!("{}", e);
            }
        }

        Commands::Cleanup => {
            if let Err(e) = cleanup_branches() {
                eprintln!("{}", e);
            }
        }

        Commands::Analyze => {
            if let Err(e) = run_analyze() {
                eprintln!("{}", e);
            }
        }
    }
}

