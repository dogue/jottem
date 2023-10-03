use jottem::{cli::SearchArgs, path::NotePath};
use serial_test::serial;
use tempfile::{tempdir, TempDir};

fn setup() -> TempDir {
    let tmp = tempdir().expect("Failed to create temporary directory");
    std::env::set_var("JOTTEM_ROOT", tmp.path());
    std::env::set_var("JOTTEM_DB_PATH", tmp.path());
    tmp
}

#[test]
#[serial]
fn test_create_note() {
    let _tmp = setup();

    let path = NotePath::parse("test_note").unwrap();
    let note = jottem::create_note(&path, &Vec::new());

    assert!(note.is_ok_and(|n| n.title == "test_note"));
}

#[test]
#[serial]
fn test_create_note_with_tags() {
    let _tmp = setup();

    let path = NotePath::parse("test_note").unwrap();
    let note = jottem::create_note(&path, &["test_tag".into()]);

    assert!(note.is_ok_and(|n| n.tags.contains("test_tag")));
}

#[test]
#[serial]
fn test_find_note_by_title() {
    let _tmp = setup();

    let path = NotePath::parse("test_note").unwrap();
    jottem::create_note(&path, &Vec::new()).unwrap();

    let search = SearchArgs {
        path: Some("test_note".into()),
        tags: Vec::new(),
        all: false,
    };

    let result = jottem::find_notes(&search);

    assert!(result.is_ok());
}
