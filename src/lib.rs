use file::file_exists;
use path::NotePath;

pub mod cli;
pub mod config;
pub mod file;
pub mod index;
pub mod note;
pub mod path;

pub fn edit(path: &str, tags: &Vec<String>) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    if file_exists(&path) {
        open(&path)?;
    } else {
        create(&path, tags)?;
    }

    Ok(())
}

pub fn delete(path: &str) -> anyhow::Result<()> {
    let path = NotePath::parse(path)?;

    if file_exists(&path) {
        // delete file
    }

    Ok(())
}

pub fn rebuild() -> anyhow::Result<()> {
    Ok(())
}

pub fn tag(command: cli::TagCommand) -> anyhow::Result<()> {
    Ok(())
}

fn create(path: &NotePath, tags: &Vec<String>) -> anyhow::Result<()> {
    Ok(())
}

fn open(path: &NotePath) -> anyhow::Result<()> {
    let editor = config::get_editor();
    let path = path.absolute_path_with_ext();

    std::process::Command::new(editor).arg(path).status()?;

    Ok(())
}
