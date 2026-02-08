use std::process::Command;
use crate::error::GitxError;

pub fn run_git(args: &[&str]) -> Result<String, GitxError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|_| GitxError::GitNotInstalled)?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(GitxError::GitCommandFailed(err));
    }

    let out = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(out)
}

pub fn ensure_git_repo() -> Result<(), GitxError> {
    run_git(&["rev-parse", "--is-inside-work-tree"])
        .map(|_| ())
        .map_err(|_| GitxError::NotAGitRepository)
}


