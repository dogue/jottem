use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(name = "edit", alias = "e", about = "[e]dit or create a note")]
    Edit { path: String },

    #[clap(name = "delete", alias = "d", about = "[d]elete a note")]
    Delete { path: String },

    #[clap(name = "rebuild", alias = "r", about = "[r]ebuild the notes index")]
    Rebuild,

    #[clap(name = "tag", alias = "t", about = "[t]ag management")]
    Tag {
        #[clap(subcommand)]
        subcommand: TagCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum TagCommand {
    Add,
    Remove,
}
