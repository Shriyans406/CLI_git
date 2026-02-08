use std::fmt;

#[derive(Debug)]
pub enum GitxError {
    GitNotInstalled,
    NotAGitRepository,
    GitCommandFailed(String),
}

impl fmt::Display for GitxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitxError::GitNotInstalled => {
                write!(f, "Git is not installed or not available in PATH")
            }
            GitxError::NotAGitRepository => {
                write!(f, "This directory is not a Git repository")
            }
            GitxError::GitCommandFailed(msg) => {
                write!(f, "Git command failed: {}", msg)
            }
        }
    }
}

