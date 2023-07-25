use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind};
use std::fmt;

use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::{Deserialize, Serialize};
use colored::Colorize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub state: usize,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        let state = 0;
        Task { text, state, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn update_state(journal_path: PathBuf, idx: usize, state: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    if idx == 0 || idx > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    
    let created_at: DateTime<Utc> = Utc::now();
    let _ = std::mem::replace(&mut tasks[idx - 1].state, state);
    let _ = std::mem::replace(&mut tasks[idx - 1].created_at, created_at);
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    tasks.retain(|x| x.state != 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    

    let file = OpenOptions::new()
        .read(true)
        .open(journal_path)?;

    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

pub fn clear_task(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    tasks.clear();
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");

        match self.state {
            1 => write!(f, "{:<25} [{}]", self.text.strikethrough().red(), created_at),
            2 => write!(f, "{:<25} [{}]", self.text.cyan(), created_at),
            0 | _ => write!(f, "{:<25} [{}]", self.text.green(), created_at),
        }
    }
}
