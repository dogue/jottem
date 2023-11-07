use clap::Parser;

use jottem::{
    cli::{Cli, Command},
    file,
    path::NotePath,
    utils,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    utils::create_root_dir()?;

    match cli.command {
        Command::Create { path, tags } => {
            let path = NotePath::parse(&path)?;
            if file::exists(&path) {
                jottem::edit_note(Some(path.relative_path()))?;
            } else {
                let note = utils::create_note(&path, &tags)?;
                utils::open_note(&note.absolute_path)?;
            }
        }
        Command::Edit { path } => jottem::edit_note(path)?,
        Command::Find { args } => jottem::find_notes(&args)?,
        Command::Delete { path } => jottem::delete_note(&path)?,
        Command::Tag { subcommand } => jottem::manage_tags(subcommand)?,
        Command::Move {
            path,
            new_path,
            rename,
        } => jottem::move_note(&path, &new_path, rename)?,
        Command::Export => jottem::export_index()?,
    };

    Ok(())
}
