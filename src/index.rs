use rocksdb::{Options, DB};
use std::{collections::HashSet, path::Path};

use crate::{config, note::Note, path::NotePath};

#[derive(Debug)]
pub struct Index {}

impl Index {
    fn open() -> anyhow::Result<DB> {
        let db_path = config::get_db_path();
        let db_path = Path::new(&db_path);
        let mut options = Options::default();
        options.create_if_missing(true);

        let db = DB::open(&options, db_path)
            .map_err(|e| anyhow::anyhow!("Failed to open database: {e}"))?;

        Ok(db)
    }

    pub fn insert(note: &Note) -> anyhow::Result<()> {
        let (id, note) = note.serialize()?;

        Self::open()?
            .put(id.to_le_bytes(), note)
            .map_err(|e| anyhow::anyhow!("Failed to insert note into the index: {e}"))?;

        Ok(())
    }

    pub fn remove(id: u64) -> anyhow::Result<()> {
        Self::open()?
            .delete(id.to_le_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to remove note from the index: {e}"))?;

        Ok(())
    }

    pub fn get(id: u64) -> anyhow::Result<Option<Note>> {
        let record = Self::open()?
            .get(id.to_le_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to retrieve database record: {e}"))?;

        if record.is_none() {
            return Ok(None);
        }

        let note = Note::deserialize(&record.unwrap())?;

        Ok(Some(note))
    }

    pub fn get_all() -> anyhow::Result<Vec<Note>> {
        let mut notes = Vec::new();

        for record in Self::open()?.iterator(rocksdb::IteratorMode::Start) {
            let (_, value) =
                record.map_err(|e| anyhow::anyhow!("Failed to read database record: {e}"))?;

            let note = Note::deserialize(&value)?;

            notes.push(note);
        }

        Ok(notes)
    }

    pub fn find_by_title(title: &str) -> anyhow::Result<Vec<Note>> {
        Ok(Self::get_all()?
            .into_iter()
            .filter(|note| note.title == title)
            .collect())
    }

    pub fn find_by_path(path: &NotePath) -> anyhow::Result<Vec<Note>> {
        Ok(Self::get_all()?
            .into_iter()
            .filter(|note| note.relative_path == path.relative_path())
            .collect())
    }

    pub fn find_by_tags(tags: &[String]) -> anyhow::Result<Vec<Note>> {
        let tags: HashSet<_> = tags.iter().collect();

        Ok(Self::get_all()?
            .into_iter()
            .filter(|n| n.tags.iter().any(|s| tags.contains(s)))
            .collect())
    }

    pub fn add_tags(id: u64, tags: &[String]) -> anyhow::Result<()> {
        if let Some(mut note) = Self::get(id)? {
            note.add_tags(tags);
            Self::insert(&note)?;
        }

        Ok(())
    }

    pub fn remove_tags(id: u64, tags: &[String]) -> anyhow::Result<()> {
        if let Some(mut note) = Self::get(id)? {
            note.remove_tags(tags);
            Self::insert(&note)?;
        }

        Ok(())
    }
}
