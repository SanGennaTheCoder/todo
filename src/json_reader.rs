use std::fs::File;
use std::io::{self, BufReader};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Completed,
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    content: String,
    status: TaskStatus,
}

pub fn read_json(path: &str) -> io::Result<Vec<Task>> {
    let file = File::open(path);

    let tasks = match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    };

    Ok(tasks)
}

pub fn write_json(tasks: &Vec<Task>, path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;

    serde_json::to_writer_pretty(file, tasks).expect("Failed to write JSON");

    Ok(())
}
