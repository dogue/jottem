use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::path::NotePath;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub path: String,
    pub title: String,
    pub created: String,
    pub modified: String,
    pub tags: Vec<String>,
}

impl Note {
    pub fn new(path: &NotePath) -> Self {
        todo!()
    }

    pub fn id(&self) -> u64 {
        let id = self.path.clone();
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.into())
    }

    pub fn serialize(self) -> anyhow::Result<(u64, Vec<u8>)> {
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
