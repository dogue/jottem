use std::{fs::OpenOptions, io, path::Path};

use crate::path::NotePath;

pub fn create_file(note_path: &NotePath) -> anyhow::Result<()> {
    let path = note_path.absolute_path_with_ext();
    let path = Path::new(&path);

    if note_path.has_parent() {
        create_parent_path(&note_path)?;
    }

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| anyhow::anyhow!("Failed to create new note file: {e}"))?;

    Ok(())
}

pub fn delete_file(note_path: &NotePath) -> anyhow::Result<()> {
    let path = note_path.absolute_path_with_ext();
    let path = Path::new(&path);

    std::fs::remove_file(path).map_err(|e| anyhow::anyhow!("Failed to remove note file: {e}"))?;

    if note_path.has_parent() {
        let entries: Vec<_> = std::fs::read_dir(note_path.absolute_parent().unwrap())?
            .collect::<Result<_, io::Error>>()?;

        if entries.is_empty() {
            let path = note_path.absolute_parent().unwrap();
            let path = Path::new(&path);

            std::fs::remove_dir(path)
                .map_err(|e| anyhow::anyhow!("Failed to remove empty directory: {e}"))?;
        }
    }

    Ok(())
}

pub fn file_exists(note_path: &NotePath) -> bool {
    let path = note_path.absolute_path_with_ext();
    let path = Path::new(&path);

    path.exists()
}

fn create_parent_path(note_path: &NotePath) -> anyhow::Result<()> {
    if !note_path.has_parent() {
        return Ok(());
    }

    let path = note_path.absolute_parent().unwrap();
    let path = Path::new(&path);

    std::fs::create_dir_all(path)
        .map_err(|e| anyhow::anyhow!("Failed to create parent path: {e}"))?;

    Ok(())
}
