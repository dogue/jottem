use cli::{SearchArgs, TagCommand};
use colored::Colorize;
use index::Index;
use path::NotePath;

pub mod cli;
pub mod config;
pub mod file;
pub mod index;
pub mod note;
pub mod path;
pub mod prompt;
pub mod tags;
pub mod utils;

/// Opens a note in the user's editor per the $EDITOR variable
///
/// * `path` - raw input from the user such as `foo/bar`
///
/// After the editor is closed, we update the modified time on the note
/// and then update the record in the index.
pub fn edit_note(path: Option<String>) -> anyhow::Result<()> {
    let mut note = if path.is_none() {
        let mut notes = Index::get_all()?;
        let options = notes
            .iter()
            .map(|n| n.relative_path.as_str())
            .collect::<Vec<&str>>();

        let selection = prompt::select_fuzzy(&options)?;
        notes.swap_remove(selection)
    } else {
        let path = path.unwrap();
        utils::get_note(&path, true)?
    };

    if utils::open_note(&note.absolute_path)? {
        note.modified = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Index::insert(&note)?;
    }

    Ok(())
}

/// Collects a list of notes based on user-specified search parameters.
///
/// * `args` - Search parameters provided by the user. See [cli::SearchArgs]
///
/// The collected notes are displayed as an ASCII table with the relative
/// path to the note and the last modified time.
pub fn find_notes(args: &SearchArgs) -> anyhow::Result<()> {
    let notes = {
        if args.path.is_some() {
            let path = args.path.as_ref().unwrap();

            let path: &str = path;
            let path = NotePath::parse(path)?;

            if path.has_parent() {
                Index::find_by_path(&path)?
            } else {
                Index::find_by_title(&path.title)?
            }
        } else if !args.tags.is_empty() {
            let tags: &[String] = &args.tags;
            Index::find_by_tags(tags)?
        } else if args.all {
            Index::get_all()?
        } else {
            println!("{}", "Found 0 matching notes".bright_red());
            return Ok(());
        }
    };

    let table = utils::build_table(notes);

    println!("{table}");

    Ok(())
}

/// Deletes a note both from disk and the index.
///
/// * `path` - raw input from the user such as `foo/bar`
pub fn delete_note(path: &str) -> anyhow::Result<()> {
    let note = utils::get_note(path, false)?;
    let path = NotePath::from_note(&note)?;

    file::delete_file(&path)?;

    Index::remove(note.id())?;

    Ok(())
}

/// Triggers the appropriate tag management action.
pub fn manage_tags(command: cli::TagCommand) -> anyhow::Result<()> {
    match command {
        TagCommand::Add { path, tags } => tags::add_tags(&path, &tags)?,
        TagCommand::Remove { path, tags } => tags::remove_tags(&path, &tags)?,
    }

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
    let new_title = new_title.replace('/', "");
    let mut note = utils::get_note(path, false)?;
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

    Index::insert(&note)?;
    Index::remove(id)?;

    file::rename_file(&old_path, &new_path)?;

    Ok(())
}

// TODO: refactor and document this
pub fn move_note(path: &str, new_path: &str, rename: bool) -> anyhow::Result<()> {
    if rename {
        rename_note(path, new_path)?;
        return Ok(());
    }

    let mut note = utils::get_note(path, false)?;

    let id = note.id();

    let old_path = NotePath::from_note(&note)?;
    let new_path = NotePath::parse(new_path)?;

    note.relative_path = new_path.relative_path();
    note.absolute_path = new_path.absolute_path_with_ext();
    note.title = new_path.title.clone();

    file::move_file(&old_path, &new_path)?;

    Index::insert(&note)?;
    Index::remove(id)?;

    Ok(())
}

/// Prints out the entire index as JSON
///
/// This is currently here for debugging purposes, but may serve
/// as part of a backup/restore feature in the future.
pub fn export_index() -> anyhow::Result<()> {
    let notes = Index::get_all()?;

    let export = serde_json::to_string(&notes)?;
    println!("{export}");

    Ok(())
}

#[cfg(test)]
mod test {
    /*! All tests must be marked with the `#[serial]` attribute.
        This prevents parallel tests from clobbering each other's
        temporary env. Each test gets an isolated, randomized
        tempdir. Use `let _tmp = setup();` to keep the tempdir
        until the test completes. Cleanup is automatic once
        `_tmp` is dropped.
    */

    use serial_test::serial;
    use std::path::Path;
    use tempfile::{tempdir, TempDir};

    use super::*;

    fn setup() -> TempDir {
        let tmp = tempdir().expect("Failed to create temporary directory");
        std::env::set_var("JOTTEM_ROOT", tmp.path());
        std::env::set_var("JOTTEM_DB_PATH", tmp.path());
        std::env::set_var("EDITOR", "true"); // successfully does nothing
        tmp
    }

    #[test]
    #[serial]
    fn test_create_note() {
        let tmp = setup();
        let tests = ["foo", "bar/baz", "far/boo/faz"];

        for test in tests {
            let input = test;
            let abs_path = Path::new(tmp.path()).join(format!("{input}.md"));
            dbg!(&abs_path);

            let creation = edit_note(Some(input.to_string()));
            assert!(creation.is_ok());

            let exists = std::fs::File::open(abs_path);
            assert!(exists.is_ok());
        }
    }
}
