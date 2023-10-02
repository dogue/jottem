use std::{fs::OpenOptions, io, path::Path};

use crate::path::NotePath;

pub fn create_file(note_path: &NotePath) -> anyhow::Result<()> {
    let path = note_path.absolute_path_with_ext();
    let path = Path::new(&path);

    if note_path.has_parent() {
        create_parent_path(note_path)?;
    }

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to create new note file: {}: {e}",
                path.to_string_lossy()
            )
        })?;

    Ok(())
}

pub fn delete_file(note_path: &NotePath) -> anyhow::Result<()> {
    let path = note_path.absolute_path_with_ext();
    let path = Path::new(&path);

    std::fs::remove_file(path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to remove note file: {}: {e}",
            path.to_string_lossy()
        )
    })?;

    if note_path.has_parent() {
        let entries: Vec<_> = std::fs::read_dir(note_path.absolute_parent().unwrap())?
            .collect::<Result<_, io::Error>>()?;

        if entries.is_empty() {
            let path = note_path.absolute_parent().unwrap();
            let path = Path::new(&path);

            std::fs::remove_dir(path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to remove empty directory: {}: {e}",
                    path.to_string_lossy()
                )
            })?;
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

    std::fs::create_dir_all(path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to create parent path: {}: {e}",
            path.to_string_lossy()
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;
    use std::fs::metadata;
    use tempfile::TempDir;

    // Filesystem operation tests are run serially
    // due to depending on the JOTTEM_ROOT env var
    // being set to a temporary location.
    // Each test gets a separate randomized tmp dir
    // for isolation.

    fn setup() -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        std::env::set_var("JOTTEM_ROOT", tmp.path());
        return tmp;
    }

    #[test]
    #[serial]
    fn test_create_file() {
        let tmp = setup();

        let path = NotePath::parse("test").expect("Failed to parse NotePath from str");
        create_file(&path).expect("Failed to create test file inside tmp directory");

        let file =
            metadata(tmp.path().join("test.md")).expect("Failed to get temporary file metadata");
        assert!(file.is_file());
    }

    #[test]
    #[serial]
    fn test_create_parent_path() {
        let tmp = setup();

        let path = NotePath::parse("parent/test").expect("Failed to parse NotePath from str");
        create_parent_path(&path).expect("Failed to create test directory inside tmp directory");

        let dir = metadata(tmp.path().join("parent"))
            .expect("Failed to get temporary directory metadata");
        assert!(dir.is_dir());
    }

    #[test]
    #[serial]
    fn test_delete_file_and_empty_dirs() {
        let tmp = setup();

        let path =
            NotePath::parse("deep/parent/path/test").expect("Failed tp parse NotePath from str");
        create_file(&path).expect("Failed to create test files inside tmp directory");

        // create a second file inside of `deep/parent` to ensure it doesn't delete non-empty dirs
        let path2 =
            NotePath::parse("deep/parent/test2").expect("Failed to parse NotePath from str");
        create_file(&path2).expect("Failed to create test file inside tmp directory");

        // After this operation, we expect `path` to be removed as it's empty.
        // `deep/parent` should still exist and contain the second file
        delete_file(&path).expect("Failed to delete test file");

        let meta = metadata(tmp.path().join("deep")).expect("deep");
        assert!(meta.is_dir());

        let meta = metadata(tmp.path().join("deep/parent")).expect("deep/parent");
        assert!(meta.is_dir());

        let meta = metadata(tmp.path().join("deep/parent/test2.md")).expect("deep/parent/test2");
        assert!(meta.is_file());

        let meta = metadata(tmp.path().join("deep/parent/path"));
        assert!(meta.is_err()); // should be a NotFound error
    }
}
