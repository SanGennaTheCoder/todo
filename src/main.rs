mod json;
use json::{Task, TaskStatus, read_json, write_json};
use std::io;

const JSON_PATH: &str = "tasks.json";

fn main() {
    loop {
        println!("Todo task manager, type the number of corresponding action");
        println!("[1] new task");
        println!("[2] read tasks");
        println!("[3] delete a task");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.is_empty() {
            println!("Error: command can't be empty");
            continue;
        }

        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match choice {
            1 => add_task(),
            2 => read_tasks(),
            3 => delete_task(),
            _ => println!("Please enter a valid option"),
        }
    }
}

fn add_task() {
    let mut tasks = read_json(JSON_PATH).expect("Failed to read tasks");

    println!("Enter task description:");
    let mut content = String::new();
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");
    let content = content.trim().to_string();

    let new_task = Task {
        content,
        status: TaskStatus::Pending,
    };

    tasks.push(new_task);
    write_json(&tasks, JSON_PATH).expect("Failed to write tasks");

    println!("Task added successfully!");
}

fn read_tasks() {
    let tasks = read_json(JSON_PATH).expect("Failed to read tasks");

    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    println!("Your tasks:");
    for (i, task) in tasks.iter().enumerate() {
        let status = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Completed",
        };
        println!("{}: [{}] {}", i + 1, status, task.content);
    }
}

fn delete_task() {
    let mut tasks = read_json(JSON_PATH).expect("Failed to read tasks");

    if tasks.is_empty() {
        println!("No tasks to delete.");
        return;
    }

    println!("Select a task to delete:");
    for (i, task) in tasks.iter().enumerate() {
        let status = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Completed",
        };
        println!("{}: [{}] {}", i + 1, status, task.content);
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= tasks.len() => num - 1,
        _ => {
            println!("Invalid task number.");
            return;
        }
    };

    let removed_task = tasks.remove(index);
    write_json(&tasks, JSON_PATH).expect("Failed to write tasks");

    println!("Deleted task: {}", removed_task.content);
}
