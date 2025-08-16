use std;
mod json_reader;

const JSON_PATH: &str = "tasks.json";

fn main() {
    loop {
        println!("Todo task manager, type the number of corresponding action");
        println!("[1] new task");
        println!("[2] read tasks");
        println!("[3] delete a task");

        let mut input = String::new();

        std::io::stdin()
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

        drop(input); // God forbid im spiraling into this habit of dropping things even tho rust drops them automatically at the end of scope, im gonna go crazy...

        match choice {
            1 => {
                // TODO: add functionality to write json task
            }

            2 => {
                // TODO: add functionality to read the json tasks and display them formatted and readable
            }

            3 => {
                // TODO: add functionality to read the json tasks and find the specific task the user wants to remove
            }

            _ => {
                println!("Please enter a valid option");
            }
        }
    }
}

fn get_task() {}
