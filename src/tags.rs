use crate::{index::INDEX, utils};

/// Adds one or more tags to an existing note.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
pub fn add_tags(path: &str, tags: &[String]) -> anyhow::Result<()> {
    let note = utils::get_note(path, true)?;

    let id = note.id();

    INDEX.add_tags(id, tags)?;

    Ok(())
}

/// Removes one or more tags from an existing note.
///
/// * `path` - raw input from the user such as `foo/bar`
/// * `tags` - a slice of String representing tags given by the user
pub fn remove_tags(path: &str, tags: &[String]) -> anyhow::Result<()> {
    let note = utils::get_note(path, false)?;

    let id = note.id();

    INDEX.remove_tags(id, tags)?;

    Ok(())
}
