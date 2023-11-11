use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use colored::Colorize;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};

use crate::{config, file, index::Index, note::Note, path::NotePath, prompt};

/// Creates the root note directory and initializes it as a git repository
///
/// Initializing a git repo in the root notes directory enables the use
/// of [Marksman](https://github.com/artempyanykh/marksman) if the user has
/// it installed.
pub fn create_root_dir() -> anyhow::Result<()> {
    std::fs::create_dir_all(config::get_root())?;

    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(config::get_root())?;

    _ = std::process::Command::new("git").arg("init").output()?;

    std::env::set_current_dir(cwd)?;

    Ok(())
}

/// Creates an ASCII table for displaying a collection of notes.
pub fn build_table(notes: Vec<Note>) -> String {
    let mut table = Table::new();

    // Creates a simple ASCII table with the outer borders removed (aesthetic choice)
    table
        .load_preset(ASCII_MARKDOWN)
        .set_style(comfy_table::TableComponent::LeftBorder, '\0')
        .set_style(comfy_table::TableComponent::RightBorder, '\0')
        .set_style(comfy_table::TableComponent::LeftHeaderIntersection, '\0')
        .set_style(comfy_table::TableComponent::RightHeaderIntersection, '\0');

    // A splash of color on the headings for extra clarity. May be changed later.
    table.set_header(vec![
        Cell::new("Note").fg(comfy_table::Color::Cyan),
        Cell::new("Modified Time").fg(comfy_table::Color::Cyan),
    ]);

    for note in notes {
        table.add_row(vec![note.relative_path, note.modified]);
    }

    table.to_string()
}

/// Disambiguates user input into a single note, optionally creating a new note
/// if no existing notes matched the input.
///
/// * `path` - raw user input such as `foo/bar`
/// * `create_if_empty` - a boolean signifying whether we should prompt the user
/// to create a new note if none of the existing notes match the input.
///
/// If `create_if_empty` is true and no existing notes match the user input,
/// we prompt the user to ask if they wish to create a new empty note.
/// If `create_if_empty` is false and no existing notes match the user input,
/// we notify the user that no notes were found and exit gracefully.
///
/// Some actions such as deleting a note don't make sense to prompt for creation.
pub fn get_note(path: &str, create_if_empty: bool) -> anyhow::Result<Note> {
    let path = NotePath::parse(path)?;

    let mut matches = if path.has_parent() {
        Index::find_by_path(&path)?
    } else {
        Index::find_by_title(&path.title)?
    };

    let note = match matches.len() {
        0 => {
            if create_if_empty && prompt::no_matches()? {
                let tags = Vec::new();
                create_note(&path, &tags)?
            } else {
                println!("{}", "Found 0 matching notes".bright_red());
                std::process::exit(0);
            }
        }
        1 => matches.pop().unwrap(),
        _ => {
            let options = matches
                .iter()
                .map(|n| n.relative_path.as_str())
                .collect::<Vec<&str>>();

            let selection = prompt::multiple_matches(&options)?;
            matches.swap_remove(selection)
        }
    };

    Ok(note)
}

/// Creates a new note both on disk and in the index.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
pub fn create_note(path: &NotePath, tags: &[String]) -> anyhow::Result<Note> {
    let note = Note::new(path, tags);

    file::create_file(path)?;

    Index::insert(&note)?;

    Ok(note)
}

/// Opens a note in the user's editor per the `$EDITOR` variable
///
/// * `path` - absolute disk path (with `.md` extension) to a note
///
/// Returns a `true` if the file changed on disk.
///
/// Before opening the file, we store the current working directory
/// and change to the root notes directory. This enables some nice
/// features from Marksman (see [`create_root_dir`]). After the editor
/// is closed, we restore the original working directory.
pub fn open_note(path: &str) -> anyhow::Result<bool> {
    let editor = config::get_editor();

    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(config::get_root())?;

    let pre_hash = hash_note(path)?;
    std::process::Command::new(editor).arg(path).status()?;
    let post_hash = hash_note(path)?;

    std::env::set_current_dir(cwd)?;

    Ok(pre_hash != post_hash)
}

fn hash_note(path: &str) -> anyhow::Result<u64> {
    let contents = std::fs::read_to_string(path)?;
    let mut hasher = DefaultHasher::new();
    contents.hash(&mut hasher);

    Ok(hasher.finish())
}
