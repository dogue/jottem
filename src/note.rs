use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::path::NotePath;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    /// Absolute filesystem path (including `.md`)
    pub absolute_path: String,
    /// Relative path inside the root directory (such as `foo/bar`)
    pub relative_path: String,
    pub title: String,
    pub created: String,
    pub modified: String,
    pub tags: HashSet<String>,
}

impl Note {
    pub fn new(note_path: &NotePath, note_tags: &[String]) -> Self {
        let absolute_path = note_path.absolute_path_with_ext();
        let relative_path = note_path.relative_path();
        let title = note_path.title();
        let created = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let modified = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let tags = HashSet::new();

        let mut note = Self {
            absolute_path,
            relative_path,
            title,
            created,
            modified,
            tags,
        };

        note.add_tags(note_tags);

        note
    }

    pub fn id(&self) -> u64 {
        let id = &self.absolute_path;
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn add_tags(&mut self, tags: &[String]) {
        for tag in tags {
            self.tags.insert(tag.to_string());
        }
    }

    pub fn remove_tags(&mut self, tags: &[String]) {
        self.tags.retain(|tag| !tags.contains(tag));
    }

    pub fn serialize(&self) -> anyhow::Result<(u64, Vec<u8>)> {
        let id = self.id();
        let serialized = bincode::serialize(&self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize note: {e}"))?
            .to_vec();

        Ok((id, serialized))
    }

    pub fn deserialize(record: &[u8]) -> anyhow::Result<Self> {
        let note = bincode::deserialize(record)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize note: {e}"))?;

        Ok(note)
    }
}
