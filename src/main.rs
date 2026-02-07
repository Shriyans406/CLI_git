mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

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

