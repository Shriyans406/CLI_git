use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};

pub fn select_commit_type() -> String {
    let types = vec!["feat", "fix", "docs", "refactor", "test", "chore"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commit type")
        .items(&types)
        .default(0)
        .interact()
        .unwrap();

    types[selection].to_string()
}

pub fn input_scope() -> String {
    Input::new()
        .with_prompt("Enter scope (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

pub fn input_message() -> String {
    Input::new()
        .with_prompt("Enter commit message")
        .interact_text()
        .unwrap()
}

pub fn confirm_commit() -> bool {
    Confirm::new()
        .with_prompt("Proceed with commit?")
        .default(true)
        .interact()
        .unwrap()
}

