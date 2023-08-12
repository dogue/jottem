use dialoguer::Select;
use file::delete_file;
use index::Index;
use lazy_static::lazy_static;
use note::Note;
use path::NotePath;

pub mod cli;
pub mod config;
pub mod file;
pub mod index;
pub mod note;
pub mod path;

lazy_static! {
    static ref INDEX: Index = Index::open().unwrap();
}

pub fn edit(path: &str, tags: &Vec<String>) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    let matches = INDEX
        .find(&path)?
        .into_iter()
        .filter(|n| n.relative_path == path.relative_path())
        .collect::<Vec<Note>>();

    let note = match matches.len() {
        0 => create(&path, tags)?,
        1 => matches[0].clone(),
        _ => {
            let mut selections: Vec<String> =
                matches.iter().map(|n| n.relative_path.clone()).collect();
            selections.push("create new".into());
            let selection = Select::new()
                .with_prompt("Multiple matches found. Please choose one")
                .default(0)
                .items(&selections)
                .interact()
                .unwrap();

            if selection < matches.len() {
                matches[selection].clone()
            } else {
                create(&path, tags)?
            }
        }
    };

    open(&note.absolute_path)?;

    Ok(())
}

pub fn delete(path: &str) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    let matches = INDEX.find(&path)?;

    let note = match matches.len() {
        1 => matches[0].clone(),
        _ => select(&matches)?,
    };

    INDEX.remove(note.id())?;
    delete_file(&NotePath::from_note(&note)?)?;

    Ok(())
}

pub fn rebuild() -> anyhow::Result<()> {
    Ok(())
}

pub fn tag(_command: cli::TagCommand) -> anyhow::Result<()> {
    Ok(())
}

fn create(path: &NotePath, tags: &Vec<String>) -> anyhow::Result<Note> {
    let note = Note::new(path, tags);
    INDEX.insert(note.clone())?;

    file::create_file(path)?;

    Ok(note)
}

fn open(path: &str) -> anyhow::Result<()> {
    let editor = config::get_editor();

    std::process::Command::new(editor).arg(path).status()?;

    Ok(())
}

fn select(matches: &Vec<Note>) -> anyhow::Result<Note> {
    let mut selections = Vec::new();
    for note in matches {
        selections.push(&note.relative_path);
    }

    let selection = Select::new()
        .with_prompt("Multiple matches found. Please choose one")
        .default(0)
        .items(&selections)
        .interact()
        .unwrap();

    Ok(matches[selection].clone())
}
