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
            let note = jottem::create(&path, &tags)?;
            jottem::open(&note.absolute_path)?;
        }
        Command::Edit { path } => jottem::edit(&path)?,
        Command::Find { args } => jottem::find(&args)?,
        Command::Delete { path } => jottem::delete(&path)?,
        Command::Tag { subcommand } => jottem::tag(subcommand)?,
        Command::Rebuild => jottem::rebuild()?,
        Command::Export => jottem::export()?,

        #[cfg(feature = "nuke")]
        Command::Nuke => jottem::nuke()?,
    };

    Ok(())
}
