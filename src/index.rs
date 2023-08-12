use std::path::Path;

use rocksdb::{Options, DB};

use crate::{config, note::Note};

pub struct Index {
    db: DB,
}

impl Index {
    pub fn open() -> anyhow::Result<Self> {
        let db_path = config::get_db_path();
        let db_path = Path::new(&db_path);
        let mut options = Options::default();
        options.create_if_missing(true);

        let db = DB::open(&options, &db_path)
            .map_err(|e| anyhow::anyhow!("Failed to open database: {e}"))?;

        Ok(Self { db })
    }

    pub fn insert(&self, note: &Note) -> anyhow::Result<()> {
        let (id, note) = note.serialize()?;

        self.db
            .put(id.to_le_bytes(), note)
            .map_err(|e| anyhow::anyhow!("Failed to insert note into the index: {e}"))?;

        Ok(())
    }

    pub fn remove(&self, id: u64) -> anyhow::Result<()> {
        self.db
            .delete(id.to_be_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to remove note from the index: {e}"))?;

        Ok(())
    }

    pub fn get(&self, id: u64) -> anyhow::Result<Option<Note>> {
        let record = self
            .db
            .get(id.to_le_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to retrieve database record: {e}"))?;

        if record.is_none() {
            return Ok(None);
        }

        let note = Note::deserialize(&record.unwrap())?;

        Ok(Some(note))
    }

    pub fn get_all(&self) -> anyhow::Result<Vec<Note>> {
        let mut notes = Vec::new();

        for record in self.db.iterator(rocksdb::IteratorMode::Start) {
            let (_, value) =
                record.map_err(|e| anyhow::anyhow!("Failed to database record: {e}"))?;

            let note = Note::deserialize(&value)?;

            notes.push(note);
        }

        Ok(notes)
    }
}
