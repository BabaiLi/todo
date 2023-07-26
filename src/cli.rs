use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Action {
    Add(SubAdd),
    Update(SubUpdate),
    Done(SubDone),
    List(SubList),
    Clear(SubClear),
}

/// Add new tasks
#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
pub struct SubAdd {
    #[argh(positional)]
    /// input text,
    /// example: todo add apple banana cat elephant
    pub text: Vec<String>,
}

/// Update task state
#[derive(FromArgs)]
#[argh(subcommand, name = "update")]
pub struct SubUpdate {
    #[argh(positional)]
    /// tasks index
    pub idx: usize,
    #[argh(positional)]
    /// task state,
    /// 0 for new/incomplete, 1 for complete, 2 for suspend
    pub state: usize,
}

/// Marks task as done
#[derive(FromArgs)]
#[argh(subcommand, name = "done")]
pub struct SubDone {}

/// List all tasks
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
pub struct SubList {}

/// Clear all tasks
#[derive(FromArgs)]
#[argh(subcommand, name = "clear")]
pub struct SubClear {}

/// A command-line app written in Rust
#[derive(FromArgs)]
pub struct CommandLineArgs {
    #[argh(subcommand)]
    pub action: Action,
    /// input file name that can be wriiten
    #[argh(option)]
    pub file: Option<PathBuf>,
}