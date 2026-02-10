use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use dialoguer::MultiSelect;

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

pub fn select_branches(branches: &[String]) -> Vec<String> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select branches to delete")
        .items(branches)
        .interact()
        .unwrap();

    selections
        .into_iter()
        .map(|i| branches[i].clone())
        .collect()
}
