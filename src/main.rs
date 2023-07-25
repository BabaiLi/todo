mod tasks;
mod cli;
use cli::{Action::*, CommandLineArgs};

use std::path::PathBuf;
use std::fs::File;

use structopt::StructOpt;
use anyhow;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".todo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        file,
    } = CommandLineArgs::from_args();

    let journal_file = file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    if let true = !journal_file.exists() {
        File::create(&journal_file)?;
    }

    match action {
        Add { text } => tasks::add_task(journal_file, text),
        Update { idx, state } => tasks::update_state(journal_file, idx, state),
        List => tasks::list_tasks(journal_file),
        Done => tasks::complete_task(journal_file),
        Clear => tasks::clear_task(journal_file),
    }?;
    Ok(())
}
