use cli::{SearchArgs, TagCommand};
use colored::Colorize;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use file::{delete_file, move_file, rename_file};
use index::Index;
use note::Note;
use path::NotePath;

pub mod cli;
pub mod config;
pub mod file;
pub mod index;
pub mod note;
pub mod path;
pub mod prompt;

/// Opens a note in the user's editor per the $EDITOR variable
///
/// * `path` - raw input from the user such as `foo/bar`
///
/// After the editor is closed, we update the modified time on the note
/// and then update the record in the index.
pub fn edit_note(path: Option<String>) -> anyhow::Result<()> {
    let mut note = if path.is_none() {
        let mut notes = Index::open()?.get_all()?;
        let options = notes
            .iter()
            .map(|n| n.relative_path.as_str())
            .collect::<Vec<&str>>();

        let selection = prompt::select_fuzzy(&options)?;
        notes.swap_remove(selection)
    } else {
        let path = path.unwrap();
        get_note(&path, true)?
    };

    open_note(&note.absolute_path)?;

    note.modified = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let index = Index::open()?;
    index.insert(&note)?;

    Ok(())
}

/// Collects a list of notes based on user-specified search parameters.
///
/// * `args` - Search parameters provided by the user. See [cli::SearchArgs]
///
/// The collected notes are displayed as an ASCII table with the relative
/// path to the note and the last modified time.
pub fn find_notes(args: &SearchArgs) -> anyhow::Result<()> {
    let index = Index::open()?;

    let notes = {
        if args.path.is_some() {
            let path = args.path.as_ref().unwrap();

            let path: &str = path;
            let path = NotePath::parse(path)?;

            if path.has_parent() {
                index.find_by_path(&path)?
            } else {
                index.find_by_title(&path.title)?
            }
        } else if !args.tags.is_empty() {
            let tags: &[String] = &args.tags;
            index.find_by_tags(tags)?
        } else if args.all {
            index.get_all()?
        } else {
            println!("{}", "Found 0 matching notes".bright_red());
            return Ok(());
        }
    };

    let table = build_table(notes);

    println!("{table}");

    Ok(())
}

/// Deletes a note both from disk and the index.
///
/// * `path` - raw input from the user such as `foo/bar`
pub fn delete_note(path: &str) -> anyhow::Result<()> {
    let note = get_note(path, false)?;
    let path = NotePath::from_note(&note)?;

    delete_file(&path)?;

    let index = Index::open()?;
    index.remove(note.id())?;

    Ok(())
}

/// Triggers the appropriate tag management action.
pub fn manage_tags(command: cli::TagCommand) -> anyhow::Result<()> {
    match command {
        TagCommand::Add { path, tags } => add_tags(&path, &tags)?,
        TagCommand::Remove { path, tags } => remove_tags(&path, &tags)?,
    }

    Ok(())
}

/// Adds one or more tags to an existing note.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
fn add_tags(path: &str, tags: &[String]) -> anyhow::Result<()> {
    let note = get_note(path, true)?;

    let id = note.id();

    let index = Index::open()?;
    index.add_tags(id, tags)?;

    Ok(())
}

/// Removes one or more tags from an existing note.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
fn remove_tags(path: &str, tags: &[String]) -> anyhow::Result<()> {
    let note = get_note(path, false)?;

    let id = note.id();

    let index = Index::open()?;
    index.remove_tags(id, tags)?;

    Ok(())
}

/// Renames a note (changes final path segment) in place.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `new_title` - the new title for the note
///
/// This function renames the note both on disk and index,
/// but does not change the relative path (except for the filename).
pub fn rename_note(path: &str, new_title: &str) -> anyhow::Result<()> {
    let new_title = new_title.replace("/", "");
    let mut note = get_note(path, false)?;
    let id = note.id();

    let old_path = NotePath::from_note(&note)?;
    let new_path = {
        if old_path.has_parent() {
            format!("{}/{}", old_path.relative_parent().unwrap(), new_title)
        } else {
            new_title
        }
    };

    let new_path = NotePath::parse(&new_path)?;

    note.relative_path = new_path.relative_path();
    note.absolute_path = new_path.absolute_path_with_ext();
    note.title = new_path.title.to_string();

    let index = Index::open()?;
    index.insert(&note)?;
    index.remove(id)?;

    rename_file(&old_path, &new_path)?;

    Ok(())
}

// TODO: refactor and document this
pub fn move_note(path: &str, new_path: &str, rename: bool) -> anyhow::Result<()> {
    if rename {
        rename_note(path, new_path)?;
        return Ok(());
    }

    let mut note = get_note(path, false)?;

    let id = note.id();

    let old_path = NotePath::from_note(&note)?;
    let new_path = NotePath::parse(new_path)?;

    note.relative_path = new_path.relative_path();
    note.absolute_path = new_path.absolute_path_with_ext();
    note.title = new_path.title.clone();

    move_file(&old_path, &new_path)?;

    let index = Index::open()?;
    index.insert(&note)?;
    index.remove(id)?;

    Ok(())
}

/// Prints out the entire index as JSON
///
/// This is currently here for debugging purposes, but may serve
/// as part of a backup/restore feature in the future.
pub fn export_index() -> anyhow::Result<()> {
    let index = Index::open()?;
    let notes = index.get_all()?;

    let export = serde_json::to_string(&notes)?;
    println!("{export}");

    Ok(())
}

/// Creates a new note both on disk and in the index.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
pub fn create_note(path: &NotePath, tags: &[String]) -> anyhow::Result<Note> {
    let note = Note::new(path, tags);

    file::create_file(path)?;

    let index = Index::open()?;
    index.insert(&note)?;

    Ok(note)
}

/// Opens a note in the user's editor per the `$EDITOR` variable
///
/// * `path` - absolute disk path (with `.md` extension) to a note
///
/// Before opening the file, we store the current working directory
/// and change to the root notes directory. This enables some nice
/// features from Marksman (see [`create_root_dir`]). After the editor
/// is closed, we restore the original working directory.
pub fn open_note(path: &str) -> anyhow::Result<()> {
    let editor = config::get_editor();

    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(config::get_root())?;

    std::process::Command::new(editor).arg(path).status()?;

    std::env::set_current_dir(cwd)?;

    Ok(())
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
    let index = Index::open()?;

    let mut matches = if path.has_parent() {
        index.find_by_path(&path)?
    } else {
        index.find_by_title(&path.title)?
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
fn build_table(notes: Vec<Note>) -> String {
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

/// Removes all note files, the root note directory, and the database.
/// Previously used for development. This command is gated behind the `nuke`
/// feature flag.
///
/// This will likely be removed in a future update.
#[cfg(feature = "nuke")]
#[deprecated(since = "0.1.2", note = "Just don't. :)")]
pub fn nuke() -> anyhow::Result<()> {
    use dialoguer::{theme::ColorfulTheme, Confirm};

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
