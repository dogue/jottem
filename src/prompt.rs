use dialoguer::{theme::ColorfulTheme, Confirm, Select};

/// Prompts the user to ask if they would like to create a new note.
pub fn no_matches() -> anyhow::Result<bool> {
    let res = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("No note found with that name. Would you like to create it now?")
        .default(true)
        .interact()?;

    return Ok(res);
}

/// Prompts the user to choose a single note from multiple matching notes.
pub fn multiple_matches(matches: &Vec<String>) -> anyhow::Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple notes found. Please choose one")
        .default(0)
        .items(&matches)
        .interact()?;

    return Ok(selection);
}
