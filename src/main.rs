use clap::Parser;

use jottem::{
    cli::{Cli, Command},
    path::NotePath,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create { path, tags } => {
            let path = NotePath::parse(&path)?;
            let note = jottem::create(&path, &tags)?;
            println!("{note:?}");
            jottem::open(&note.absolute_path)?;
        }
        Command::Edit { path } => jottem::edit(&path)?,
        Command::Delete { path } => jottem::delete(&path)?,
        Command::Rebuild => jottem::rebuild()?,
        Command::Tag { subcommand } => jottem::tag(subcommand)?,
    };

    Ok(())
}
