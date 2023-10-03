use crate::{note::Note, path::NotePath, INDEX};

pub fn by_path(path: &str) -> anyhow::Result<Vec<Note>> {
    let path = NotePath::parse(path)?;

    if path.has_parent() {
        INDEX.find_by_path(&path)
    } else {
        INDEX.find_by_title(&path.title())
    }
}

pub fn by_tags(tags: &[String]) -> anyhow::Result<Vec<Note>> {
    INDEX.find_by_tags(tags)
}

pub fn all() -> anyhow::Result<Vec<Note>> {
    INDEX.get_all()
}
