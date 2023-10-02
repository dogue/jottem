use cli::{SearchArgs, TagCommand};
use colored::*;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use file::{delete_file, rename_file};
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
    let matches = retrieve_matches(path)?;

    let note = match matches.len() {
        0 => prompt_no_matches(path)?,
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };

    open(&note.absolute_path)?;

    let mut note = INDEX.get(note.id())?.unwrap();
    note.modified = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    INDEX.insert(note)?;

    Ok(())
}

pub fn find(args: &SearchArgs) -> anyhow::Result<()> {
    let notes = {
        if let Some(path) = args.path.clone() {
            let path = NotePath::parse(&path)?;

            if path.has_parent() {
                INDEX.find_by_path(&path)?
            } else {
                INDEX.find_by_title(&path.title())?
            }
        } else if !args.tags.is_empty() {
            INDEX.find_by_tags(&args.tags)?
        } else if args.all {
            INDEX.get_all()?
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

pub fn tag(command: cli::TagCommand) -> anyhow::Result<()> {
    match command {
        TagCommand::Add { path, tags } => add_tags(&path, &tags)?,
        TagCommand::Remove { path, tags } => remove_tags(&path, &tags)?,
    }

    Ok(())
}

fn add_tags(input: &str, tags: &Vec<String>) -> anyhow::Result<()> {
    let matches = retrieve_matches(input)?;

    let note = match matches.len() {
        0 => prompt_no_matches(input)?,
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };

    let id = note.id();

    INDEX.add_tags(id, tags)?;

    Ok(())
}

fn remove_tags(input: &str, tags: &[String]) -> anyhow::Result<()> {
    let matches = retrieve_matches(input)?;

    let note = match matches.len() {
        0 => {
            println!("{}", "No notes found".bright_red());
            std::process::exit(0);
        }
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };

    let id = note.id();

    INDEX.remove_tags(id, tags)?;

    Ok(())
}

pub fn rename(path: &str, new_title: &str) -> anyhow::Result<()> {
    let matches = retrieve_matches(path)?;

    let mut note = match matches.len() {
        0 => {
            println!("{}", "No notes found".bright_red());
            std::process::exit(0);
        }
        1 => matches[0].clone(),
        _ => prompt_multiple_matches(&matches)?,
    };
    let id = note.id();

    let old_path = NotePath::from_note(&note)?;
    let new_path = {
        if old_path.has_parent() {
            format!("{}/{}", old_path.relative_parent().unwrap(), new_title)
        } else {
            new_title.to_string()
        }
    };

    let new_path = NotePath::parse(&new_path)?;

    note.relative_path = new_path.relative_path();
    note.absolute_path = new_path.absolute_path_with_ext();
    note.title = new_path.title();

    INDEX.insert(note)?;
    INDEX.remove(id)?;

    rename_file(&old_path, &new_path)?;

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

    // Setting the working directory to the notes root enables multi-file features
    // of Marksman if the notes root is a repo.
    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(config::get_root())?;

    std::process::Command::new(editor).arg(path).status()?;

    std::env::set_current_dir(cwd)?;

    Ok(())
}

fn retrieve_matches(input: &str) -> anyhow::Result<Vec<Note>> {
    let path = NotePath::parse(input)?;

    if path.has_parent() {
        INDEX.find_by_path(&path)
    } else {
        INDEX.find_by_title(&path.title())
    }
}

/// Prompts the user to choose a single note from multiple matching notes.
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

/// Prompts the user to ask if they would like to create a new note.
fn prompt_no_matches(input: &str) -> anyhow::Result<Note> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("No note found with that name. Would you like to create it now?")
        .default(true)
        .interact()?
    {
        let path = NotePath::parse(input)?;
        let tags: Vec<String> = Vec::new();
        create(&path, &tags)
    } else {
        std::process::exit(0);
    }
}

/// Creates the root note directory and initializes it as a git repository (for Marksman integration)
pub fn create_root_dir() -> anyhow::Result<()> {
    std::fs::create_dir_all(config::get_root())?;

    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(config::get_root())?;

    _ = std::process::Command::new("git").arg("init").output()?;

    std::env::set_current_dir(cwd)?;

    Ok(())
}

/// Creates an ASCII table for displaying a collection of notes.
fn build_table(notes: Vec<Note>) -> String {
    let mut table = Table::new();

    table
        .load_preset(ASCII_MARKDOWN)
        .set_style(comfy_table::TableComponent::LeftBorder, '\0')
        .set_style(comfy_table::TableComponent::RightBorder, '\0')
        .set_style(comfy_table::TableComponent::LeftHeaderIntersection, '\0')
        .set_style(comfy_table::TableComponent::RightHeaderIntersection, '\0');

    table.set_header(vec![
        Cell::new("Note").fg(comfy_table::Color::Cyan),
        Cell::new("Modified Time").fg(comfy_table::Color::Cyan),
    ]);

    for note in notes {
        table.add_row(vec![note.relative_path, note.modified]);
    }

    table.to_string()
}

/// Removes all note files, the root note directory, and the database.
/// Previously used for development. This command is gated behind the `nuke`
/// feature flag.
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
