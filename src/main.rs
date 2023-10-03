use clap::Parser;

use jottem::{
    cli::{Cli, Command},
    path::NotePath,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    jottem::create_root_dir()?;

    match cli.command {
        Command::Create { path, tags } => {
            let path = NotePath::parse(&path)?;
            let note = jottem::create_note(&path, &tags)?;
            jottem::open_note(&note.absolute_path)?;
        }
        Command::Edit { path } => jottem::edit_note(&path)?,
        Command::Find { args } => jottem::find_notes(&args)?,
        Command::Delete { path } => jottem::delete_note(&path)?,
        Command::Tag { subcommand } => jottem::manage_tags(subcommand)?,
        Command::Rename { path, new_title } => jottem::rename_note(&path, &new_title)?,
        Command::Move { path, new_path } => jottem::move_note(&path, &new_path)?,
        Command::Export => jottem::export_index()?,

        #[cfg(feature = "nuke")]
        Command::Nuke => jottem::nuke()?,
    };

    Ok(())
}
