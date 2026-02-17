mod cli;
mod git;
mod error;
mod status;
mod branch;
mod commit;
mod ui;
mod analyze;
mod remote;

use clap::Parser;
use cli::{Cli, Commands};
use git::ensure_git_repo;
use status::show_status;
use branch::{list_branches, create_branch, cleanup_branches};
use commit::run_commit;
use analyze::run as run_analyze;

fn main() {
    let cli = Cli::parse();

    // Ensure we are inside a git repository
    if let Err(e) = ensure_git_repo() {
        eprintln!("{}", e);
        return;
    }

    // 🔹 INTERACTIVE MODE (when no subcommand provided)
    if cli.command.is_none() {
        if let Some(choice) = ui::interactive_menu() {
            match choice.as_str() {
                "Status" => {
                    if let Err(e) = show_status() {
                        eprintln!("{}", e);
                    }
                }
                "Branches" => {
                    if let Err(e) = list_branches() {
                        eprintln!("{}", e);
                    }
                }
                "New Branch" => {
                    println!("Use CLI: gitx new <branch-name>");
                }
                "Commit" => {
                    if let Err(e) = run_commit() {
                        eprintln!("{}", e);
                    }
                }
                "Cleanup" => {
                    if let Err(e) = cleanup_branches() {
                        eprintln!("{}", e);
                    }
                }
                "Analyze" => {
                    if let Err(e) = run_analyze() {
                        eprintln!("{}", e);
                    }
                }
                "Push" => {
    if remote::check_remote() {
        remote::pull_changes();
        remote::push_changes();
    } else {
        println!("No remote configured.");
    }
}

                "Exit" => return,
                _ => {}
            }
        }
        return;
    }

    // 🔹 NORMAL CLI MODE (existing logic preserved)
    match cli.command.unwrap() {
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
        
        Commands::Push => {
    if remote::check_remote() {
        remote::pull_changes();
        remote::push_changes();
    } else {
        println!("No remote configured.");
    }
}

    }
}

