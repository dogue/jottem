use clap::Parser;

use jottem::cli;
use jottem::config;

fn main() {
    let _opts = cli::Cli::parse();
    let _root = config::get_root();
}
