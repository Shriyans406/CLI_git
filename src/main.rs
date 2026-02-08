mod cli;
mod git;
mod error;

use clap::Parser;
use cli::{Cli, Commands};
use git::ensure_git_repo;   // ← ADD this


fn main() {
    let cli = Cli::parse();
    
    // Ensure we are inside a git repository
if let Err(e) = ensure_git_repo() {
    eprintln!("{}", e);
    return;
}


    match cli.command {
        Commands::St => {
            println!("status command invoked");
        }
        Commands::Br => {
            println!("branch list command invoked");
        }
        Commands::New { branch } => {
            println!("create new branch: {}", branch);
        }
        Commands::Commit => {
            println!("commit assistant invoked");
        }
        Commands::Cleanup => {
            println!("branch cleanup invoked");
        }
        Commands::Analyze => {
            println!("commit analysis invoked");
        }
    }
}

