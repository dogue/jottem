use clap::Parser;

use jottem::cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    _ = match cli.command {
        Command::Edit { path, tags } => jottem::edit(&path, &tags),
        Command::Delete { path } => jottem::delete(&path),
        Command::Rebuild => jottem::rebuild(),
        Command::Tag { subcommand } => jottem::tag(subcommand),
    };

    Ok(())
}
