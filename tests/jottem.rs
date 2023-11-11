/*! All tests must be marked with the `#[serial]` attribute.
    This prevents parallel tests from clobbering each other's
    temporary env. Each test gets an isolated, randomized
    tempdir. Use `let _tmp = setup();` to keep the tempdir
    until the test completes. Cleanup is automatic once
    `_tmp` is dropped.
*/

// use std::path::Path;

// use jottem::{cli::SearchArgs, path::NotePath};
// use serial_test::serial;
// use tempfile::{tempdir, TempDir};

// fn setup() -> TempDir {
//     let tmp = tempdir().expect("Failed to create temporary directory");
//     std::env::set_var("JOTTEM_ROOT", tmp.path());
//     std::env::set_var("JOTTEM_DB_PATH", tmp.path());
//     std::env::set_var("EDITOR", "true"); // successfully does nothing
//     tmp
// }

// #[test]
// #[serial]
// fn test_create_note() {
//     let _tmp = setup();

//     let path = NotePath::parse("test_note").unwrap();
//     let note = jottem::utils::create_note(&path, &Vec::new());

//     assert!(note.is_ok_and(|n| n.title == "test_note"));
// }

// #[test]
// #[serial]
// fn test_create_note_with_tags() {
//     let _tmp = setup();

//     let path = NotePath::parse("test_note").unwrap();
//     let note = jottem::utils::create_note(&path, &["test_tag".into()]);

//     assert!(note.is_ok_and(|n| n.tags.contains("test_tag")));
// }

// #[test]
// #[serial]
// fn test_find_note_by_title() {
//     let _tmp = setup();

//     let path = NotePath::parse("test_note").unwrap();
//     jottem::utils::create_note(&path, &Vec::new()).unwrap();

//     let search = SearchArgs {
//         path: Some("test_note".into()),
//         tags: Vec::new(),
//         all: false,
//     };

//     let result = jottem::find_notes(&search);

//     assert!(result.is_ok());
// }
