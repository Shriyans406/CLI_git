use std::process::Command;
use crate::git::get_current_branch;

pub fn check_remote() -> bool {
    let output = Command::new("git")
        .args(["remote"])
        .output();

    if let Ok(out) = output {
        let remotes = String::from_utf8_lossy(&out.stdout);
        return remotes.contains("origin");
    }

    false
}

pub fn pull_changes() {
    println!("Pulling latest changes...");

    let branch = match get_current_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Could not detect branch.");
            return;
        }
    };

    let status = Command::new("git")
        .args(["pull", "origin", &branch])
        .status();

    match status {
        Ok(s) if s.success() => println!("Pull successful."),
        _ => println!("Pull failed."),
    }
}

pub fn push_changes() {
    println!("Pushing changes...");

    let branch = match get_current_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Could not detect branch.");
            return;
        }
    };

    println!("Current branch: {}", branch);

    let status = Command::new("git")
        .args(["push", "origin", &branch])
        .status();

    match status {
        Ok(s) if s.success() => println!("Push successful."),
        _ => println!("Push failed."),
    }
}

