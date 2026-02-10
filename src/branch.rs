use crate::git::run_git;
use crate::error::GitxError;
use crate::ui::select_branches;


const PROTECTED: [&str; 3] = ["main", "master", "dev"];

pub fn list_branches() -> Result<(), GitxError> {
    let output = run_git(&["branch"])?;
    println!("{}", output);
    Ok(())
}

pub fn create_branch(name: &str) -> Result<(), GitxError> {
    run_git(&["checkout", "-b", name])?;
    println!("Switched to new branch '{}'", name);
    Ok(())
}



pub fn cleanup_branches() -> Result<(), GitxError> {
    let current = run_git(&["branch", "--show-current"])?;
    let current = current.trim();

    let merged = run_git(&["branch", "--merged"])?;
    let candidates: Vec<String> = merged
        .lines()
        .map(|l| l.trim().trim_start_matches('*').trim().to_string())
        .filter(|b| !PROTECTED.contains(&b.as_str()))
        .filter(|b| b != current)
        .collect();

    if candidates.is_empty() {
        println!("No merged branches to clean up.");
        return Ok(());
    }

    let selected = select_branches(&candidates);

    if selected.is_empty() {
        println!("No branches selected.");
        return Ok(());
    }

    for branch in selected {
        run_git(&["branch", "-d", &branch])?;
        println!("Deleted branch '{}'", branch);
    }

    Ok(())
}

