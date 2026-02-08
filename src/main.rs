mod cli;
mod git;
mod error;
mod status;
mod branch;
mod commit;
mod ui;


use clap::Parser;
use cli::{Cli, Commands};
use git::ensure_git_repo;   // ← ADD this
use status::show_status;
use branch::{list_branches, create_branch};
use commit::run_commit;




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

    Commands::Cleanup => println!("branch cleanup invoked"),
    Commands::Analyze => println!("commit analysis invoked"),
}

}

