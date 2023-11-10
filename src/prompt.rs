use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Select};

/// Prompts the user to ask if they would like to create a new note.
pub fn no_matches() -> anyhow::Result<bool> {
    let res = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("No note found with that name. Would you like to create it now?")
        .default(true)
        .interact_opt()?;

    if res.is_none() {
        std::process::exit(0);
    }

    Ok(res.unwrap())
}

/// Prompts the user to choose a single note from multiple matching notes.
pub fn multiple_matches(matches: &[&str]) -> anyhow::Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple notes found. Please choose one")
        .default(0)
        .items(matches)
        .interact_opt()?;

    if selection.is_none() {
        std::process::exit(0);
    }

    Ok(selection.unwrap())
}

/// Prompts the user to choose a note with fuzzy finding
pub fn select_fuzzy(notes: &[&str]) -> anyhow::Result<usize> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(notes)
        .interact_opt()?;

    if selection.is_none() {
        // user cancelled
        std::process::exit(0);
    }

    Ok(selection.unwrap())
}
