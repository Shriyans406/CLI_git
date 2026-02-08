use crate::git::run_git;
use crate::error::GitxError;

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

