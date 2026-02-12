use std::process::Command;

pub fn check_remote() -> bool {
    let output = Command::new("git")
        .arg("remote")
        .output()
        .expect("Failed to execute git remote");

    let remotes = String::from_utf8_lossy(&output.stdout);

    !remotes.trim().is_empty()
}

pub fn add_remote(url: &str) {
    let status = Command::new("git")
        .args(["remote", "add", "origin", url])
        .status()
        .expect("Failed to add remote");

    if status.success() {
        println!("Remote added successfully.");
    } else {
        println!("Failed to add remote.");
    }
}

pub fn pull_changes() {
    let status = Command::new("git")
        .args(["pull", "origin", "main"])
        .status()
        .expect("Failed to pull changes");

    if status.success() {
        println!("⬇ Pulled latest changes.");
    } else {
        println!("⚠ Pull failed. Resolve conflicts manually.");
    }
}

pub fn push_changes() {
    let status = Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .status()
        .expect("Failed to push changes");

    if status.success() {
        println!("Changes pushed successfully.");
    } else {
        println!("Push failed.");
    }
}

