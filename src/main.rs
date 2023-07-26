mod tasks;
mod cli;
use cli::{Action::*, CommandLineArgs};

use std::path::PathBuf;

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
    } = argh::from_env();

    let journal_file = file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    match action {
        Add(add) => tasks::add_task(journal_file, add.text),
        Update(update) => tasks::update_state(journal_file, update.idx, update.state),
        List(_list) => tasks::list_tasks(journal_file),
        Done(_done) => tasks::complete_task(journal_file),
        Clear(_clear) => tasks::clear_task(journal_file),
    }?;
    Ok(())
}
