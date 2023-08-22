use clap::{command, Args, Parser, Subcommand};

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

    #[clap(name = "find", alias = "f", about = "[f]ind a note")]
    Find {
        #[command(flatten)]
        args: SearchArgs,
    },

    #[clap(name = "delete", alias = "d", about = "[d]elete a note")]
    Delete {
        #[arg(help = "note title or relative path")]
        path: String,
    },

    #[clap(name = "tag", alias = "t", about = "[t]ag management")]
    Tag {
        #[clap(subcommand)]
        subcommand: TagCommand,
    },

    #[clap(name = "rebuild", alias = "r", about = "[r]ebuild the notes index")]
    Rebuild,

    #[clap(
        name = "export",
        alias = "x",
        about = "e[x]port the notes index as JSON"
    )]
    Export,

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

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short, long, value_delimiter = ',')]
    pub tags: Vec<String>,
}
