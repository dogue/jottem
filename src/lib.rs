use cli::SearchArgs;
use colored::*;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use file::delete_file;
use index::Index;
use lazy_static::lazy_static;
use note::Note;
use path::NotePath;

pub mod cli;
pub mod config;
pub mod file;
pub mod index;
pub mod note;
pub mod path;

lazy_static! {
    static ref INDEX: Index = Index::open().unwrap();
}

pub fn edit(path: &str) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    let matches = {
        if path.has_parent() {
            INDEX.find_by_path(&path)?
        } else {
            INDEX.find_by_title(&path.title())?
        }
    };

    let note = match matches.len() {
        0 => prompt_no_matches(&path)?,
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };

    open(&note.absolute_path)?;

    let mut note = INDEX.get(note.id())?.unwrap();
    note.modified = chrono::offset::Local::now().to_string();
    INDEX.insert(note)?;

    Ok(())
}

pub fn find(args: &SearchArgs) -> anyhow::Result<()> {
    let notes = {
        if let Some(path) = args.path.clone() {
            INDEX.find_by_title(&path)?
        } else if args.tags.len() > 0 {
            INDEX.find_by_tags(&args.tags)?
        } else {
            Vec::new()
        }
    };

    let table = build_table(notes);

    println!("{table}");

    Ok(())
}

pub fn delete(path: &str) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    let matches = INDEX.find_by_title(&path.title())?;

    let note = match matches.len() {
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };

    INDEX.remove(note.id())?;
    delete_file(&NotePath::from_note(&note)?)?;

    Ok(())
}

pub fn tag(_command: cli::TagCommand) -> anyhow::Result<()> {
    Ok(())
}

pub fn rebuild() -> anyhow::Result<()> {
    Ok(())
}

pub fn export() -> anyhow::Result<()> {
    let notes = INDEX.get_all()?;
    let export = serde_json::to_string(&notes)?;

    println!("{export}");

    Ok(())
}

pub fn create(path: &NotePath, tags: &Vec<String>) -> anyhow::Result<Note> {
    let note = Note::new(path, tags);

    file::create_file(path)?;
    INDEX.insert(note.clone())?;

    Ok(note)
}

pub fn open(path: &str) -> anyhow::Result<()> {
    let editor = config::get_editor();

    std::process::Command::new(editor).arg(path).status()?;

    Ok(())
}

fn prompt_multiple_matches(matches: &Vec<Note>) -> anyhow::Result<Note> {
    let mut selections = Vec::new();
    for note in matches {
        selections.push(&note.relative_path);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple notes found. Please choose one")
        .default(0)
        .items(&selections)
        .interact()
        .unwrap();

    Ok(matches[selection].clone())
}

fn prompt_no_matches(path: &NotePath) -> anyhow::Result<Note> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("No note found with that name. Would you like to create it now?")
        .default(true)
        .interact()?
    {
        let tags: Vec<String> = Vec::new();
        create(path, &tags)
    } else {
        std::process::exit(0);
    }
}

pub fn create_root_dir() -> anyhow::Result<()> {
    std::fs::create_dir_all(config::get_root())?;

    Ok(())
}

// Uses the comfy-table crate
// keeping it here for now but I didn't like the output - too busy
/*fn build_table(notes: Vec<Note>) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Disabled)
        .set_width(40)
        .set_header(vec!["Relative Path", "Modified Time"]);

    for note in notes {
        table.add_row(vec![note.relative_path, note.modified]);
    }

    table
}

// First alternative to the comfy-table version
// isn't dynamic
fn build_table_alt(notes: Vec<Note>) -> String {
    let mut table = String::new();

    table.push_str("Relative Path\t\tModified Time\n");

    for note in notes {
        table.push_str(&format!("{}\t\t{}\n", note.relative_path, note.modified));
    }

    table.trim_end().to_string()
}*/

fn build_table(notes: Vec<Note>) -> String {
    let mut table = String::new();

    let max_len = notes
        .iter()
        .map(|n| n.relative_path.len())
        .max()
        .unwrap_or(0)
        + 4;

    table.push_str(
        &format!(
            "{:<width$}{}\n",
            "Relative Path".cyan().bold(),
            "Modified Time".cyan().bold(),
            width = max_len
        )
        .cyan(),
    );

    for note in notes {
        table.push_str(&format!(
            "{:<width$}{}\n",
            note.relative_path,
            note.modified,
            width = max_len
        ));
    }

    table.trim_end().to_string()
}

#[cfg(feature = "nuke")]
pub fn nuke() -> anyhow::Result<()> {
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("THIS WILL DELETE ALL YOUR NOTES. ARE YOU SURE? THERE IS NO UNDO")
        .default(false)
        .wait_for_newline(true)
        .interact()?
    {
        std::process::exit(0);
    }

    std::fs::remove_dir_all(config::get_root())?;
    std::fs::remove_dir_all(config::get_db_path())?;
    std::fs::create_dir(config::get_root())?;

    Ok(())
}
