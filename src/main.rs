mod tasks;
mod cli;
use tasks::Task;
use cli::{Action::*, CommandLineArgs};

use std::path::PathBuf;

use structopt::StructOpt;
use anyhow;
use colored::*;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".todo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    control::set_virtual_terminal(true).unwrap();
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
        Update { idx, state } => tasks::update_state(journal_file, idx, state),
        Clear => tasks::clear_task(journal_file),
    }?;
    Ok(())
}
