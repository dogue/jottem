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

    #[clap(name = "rename", alias = "r", about = "[r]ename a note")]
    Rename {
        #[arg(help = "note title or relative path")]
        path: String,

        #[arg(help = "new note title")]
        new_title: String,
    },

    #[clap(name = "move", alias = "m", about = "[m]ove a note")]
    Move {
        #[arg(help = "note title or relative path")]
        path: String,

        #[arg(help = "new relative path")]
        new_path: String,
    },

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

/// Search parameters used for finding notes.
///
/// * `path` (`-p`, `--path`) - a typical path such as `foo/bar`
/// * `tags` (`-t`, `--tags`) - a comma-separated list of tags
/// * `all` (`-a`, `--all`) - takes no arguments, returns all notes
///
/// All three fields (flags) are mutually exclusive.
///
/// `path` requires a path or note title (such as `foo/bar` or `baz`)
/// to be provided.
///
/// `tags` requires a comma-separated list of one or more tags to be
/// provided.
///
/// `all` requires no arguments and returns all notes
#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short, long, value_delimiter = ',')]
    pub tags: Vec<String>,

    #[arg(short, long)]
    pub all: bool,
}
