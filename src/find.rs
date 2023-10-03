use crate::{note::Note, path::NotePath, INDEX};

pub fn by_path(path: &str) -> anyhow::Result<Vec<Note>> {
    let path = NotePath::parse(path)?;

    if path.has_parent() {
        return INDEX.find_by_path(&path);
    } else {
        return INDEX.find_by_title(&path.title());
    }
}

pub fn by_tags(tags: &[String]) -> anyhow::Result<Vec<Note>> {
    return INDEX.find_by_tags(tags);
}

pub fn all() -> anyhow::Result<Vec<Note>> {
    return INDEX.get_all();
}
