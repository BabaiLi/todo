use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(about = "Add new task")]
    Add {
        #[structopt()]
        text: String,
    },
    #[structopt(about = "Update task status")]
    Update {
        #[structopt()]
        idx: usize,
        #[structopt()]
        state: usize,
    },
    #[structopt(about = "Marks task as done")]
    Done,
    #[structopt(about = "List all tasks")]
    List,
    #[structopt(about = "Clear all tasks")]
    Clear,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "To-do List",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub file: Option<PathBuf>,
}