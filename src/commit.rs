use crate::git::run_git;
use crate::error::GitxError;
use crate::ui::*;

pub fn run_commit() -> Result<(), GitxError> {
    // 1. Check staged files
    let staged = run_git(&["diff", "--cached", "--name-only"])?;

    if staged.trim().is_empty() {
        println!("No staged files found. Stage files before committing.");
        return Ok(());
    }

    // 2. Collect inputs
    let commit_type = select_commit_type();
    let scope = input_scope();
    let message = input_message();

    if message.trim().is_empty() {
        println!("Commit message cannot be empty.");
        return Ok(());
    }

    // 3. Build commit message
    let final_message = if scope.trim().is_empty() {
        format!("{}: {}", commit_type, message)
    } else {
        format!("{}({}): {}", commit_type, scope, message)
    };

    if final_message.len() > 72 {
        println!("Commit message too long (max 72 characters).");
        return Ok(());
    }

    // 4. Preview
    println!("\nCommit preview:\n{}\n", final_message);

    // 5. Confirm
    if !confirm_commit() {
        println!("Commit cancelled.");
        return Ok(());
    }

    // 6. Commit
    run_git(&["commit", "-m", &final_message])?;
    println!("Commit created successfully.");

    Ok(())
}

