use std::collections::HashMap;
use std::process::Command;

use crate::error::GitxError;

struct Commit {
    hash: String,
    author: String,
    date: String,
}

pub fn run() -> Result<(), GitxError> {
    // Run git log command
    let output = Command::new("git")
        .args([
            "log",
            "--pretty=format:%H|%an|%ad",
            "--date=short",
        ])
        .output()
        .map_err(|_| GitxError::GitCommandFailed("Failed to execute git".into()))?;

    if !output.status.success() {
        return Err(GitxError::GitCommandFailed(
            "Not a git repository or git log failed".into(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() {
        println!("No commits found in this repository.");
        return Ok(());
    }

    // Parse commits
    let mut commits = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            continue;
        }

        commits.push(Commit {
            hash: parts[0].to_string(),
            author: parts[1].to_string(),
            date: parts[2].to_string(),
        });
    }

    // Analysis
    let total_commits = commits.len();

    let mut author_count: HashMap<String, usize> = HashMap::new();
    let mut date_count: HashMap<String, usize> = HashMap::new();

    for commit in &commits {
        *author_count.entry(commit.author.clone()).or_insert(0) += 1;
        *date_count.entry(commit.date.clone()).or_insert(0) += 1;
    }

    let most_active_author = author_count
        .iter()
        .max_by_key(|(_, count)| *count);

    let busiest_day = date_count
        .iter()
        .max_by_key(|(_, count)| *count);

    // Output
    println!("\nCommit Analysis\n");
    println!("Total commits: {}\n", total_commits);

    println!("Commits by author:");
    for (author, count) in &author_count {
        println!("- {}: {}", author, count);
    }

    if let Some((author, count)) = most_active_author {
        println!("\nMost active author: {} ({} commits)", author, count);
    }

    println!("\nCommits by day:");
    for (date, count) in &date_count {
        println!("- {}: {}", date, count);
    }

    if let Some((day, count)) = busiest_day {
        println!("\nBusiest day: {} ({} commits)", day, count);
    }

    Ok(())
}

