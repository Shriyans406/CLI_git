use crate::git::run_git;
use crate::error::GitxError;

pub fn show_status() -> Result<(), GitxError> {
    let output = run_git(&["status", "--short"])?;

    if output.trim().is_empty() {
        println!("Working tree clean");
    } else {
        println!("{}", output);
    }

    Ok(())
}

