use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(name = "create", alias = "c", about = "[c]reate a new note")]
    Create {
        #[arg(help = "note title or relative path")]
        path: String,

        #[arg(
            short,
            long,
            required = false,
            value_delimiter = ',',
            help = "a list of tags to add to the note"
        )]
        tags: Vec<String>,
    },

    #[clap(name = "edit", alias = "e", about = "[e]dit a note")]
    Edit {
        #[arg(help = "note title or relative path")]
        path: String,
    },

    #[clap(name = "delete", alias = "d", about = "[d]elete a note")]
    Delete {
        #[arg(help = "note title or relative path")]
        path: String,
    },

    #[clap(name = "rebuild", alias = "r", about = "[r]ebuild the notes index")]
    Rebuild,

    #[clap(name = "tag", alias = "t", about = "[t]ag management")]
    Tag {
        #[clap(subcommand)]
        subcommand: TagCommand,
    },

    #[cfg(feature = "nuke")]
    #[clap(
        name = "nuke",
        about = "removes all notes and the index (intended for development)"
    )]
    Nuke,
}

#[derive(Debug, Subcommand)]
pub enum TagCommand {
    #[command(about = "add tags to an existing note")]
    Add {
        #[arg(help = "note title or relative path")]
        path: String,

        #[arg(help = "list of tags to add", value_delimiter = ',')]
        tags: Vec<String>,
    },

    #[command(about = "remove tags from an existing note")]
    Remove {
        #[arg(help = "note title or relative path")]
        path: String,

        #[arg(help = "list of tags to remove", value_delimiter = ',')]
        tags: Vec<String>,
    },
}
