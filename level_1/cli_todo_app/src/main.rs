use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new_task(description: String) -> Self {
        Task {
            description: description,
            done: false,
        }
    }
}

fn check_file_exists() -> bool {
    let file_exists = Path::new("./tasks.json").exists();
    if (file_exists) {
        return true;
    }
    return false;
}

// fn create_new_file() -> bool {
//     match File::create("./tasks.json") {
//         Ok(file) => {
//             println!("Created new file");
//             return true;
//         }
//         Err(_) => {
//             println!("Error while creating a new file. Try again later");
//             return false;
//         }
//     }
// }

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize the data");

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./tasks.json")
    {
        // truncate is used to clear all the data from the file before saving the new data
        Ok(file) => file,
        Err(_) => {
            println!("Unable to open the file at this moment. Try again later");
            return;
        }
    };

    file.write_all(json.as_bytes())
        .expect("Unable to write the file");
}

fn load_tasks() -> Vec<Task> {
    if !check_file_exists() {
        return Vec::new();
    }

    let mut file = File::open("./tasks.json").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file"); // converting the bytes into string and storing them in contents;

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()) // if the result is Ok/Some the return the result or else return the Vec;
}

fn main() {
    let mut tasks = load_tasks();
    
    loop {
        let mut userOperation = String::new();
        println!("Enter an operation: add, remove, list, save, exit");
        io::stdout().flush().unwrap(); // to immediately print the above output string;

        io::stdin()
            .read_line(&mut userOperation)
            .expect("User input is required");
        let user_operation = userOperation.trim();
        if user_operation.is_empty() {
            println!("User operation is required");
        };

        match user_operation {
            "add" => {
                println!("Add new task: ");
                io::stdout().flush().unwrap();
               
                let mut desc = String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("User input is required");
                let desc = desc.trim(); // shadowing
                if desc.is_empty() {
                    println!("Task canonot be empty");
                }
                tasks.push(Task::new_task({ desc.to_owned() }));
            }
            "remove" => {
                println!("Remove a task by entering its Index(0..x)");
                io::stdout().flush().unwrap();
                let mut idx = String::new();
                io::stdin()
                    .read_line(&mut idx)
                    .expect("User input is required");
                if idx.trim().is_empty() {
                    println!("User input is required");
                }
                let index = match idx.trim().parse::<usize>() {
                    Ok(ind) => {
                        if ind < 0 || ind > tasks.len() {
                            println!("Invalid number");
                        }
                        tasks.remove(ind);
                        save_tasks(&tasks);
                    }
                    Err(_) => {
                        println!("Invalid number");
                    }
                };
            }
            "list" => {
                if tasks.is_empty() {
                    println!("No tasks yet");
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        let status = if task.done { "[✓]" } else { "[ ]" };
                        println!("{} {} {}", i, status, task.description);
                    }
                }
            }
            "save" => {
                save_tasks(&tasks);
            }
            "done" => {
                print!("Enter task index to mark as done: ");
                io::stdout().flush().unwrap();

                let mut idx_str = String::new();
                io::stdin()
                    .read_line(&mut idx_str)
                    .expect("Failed to read input");
                if idx_str.trim().is_empty() {
                    println!("Invalid number")
                }
                match idx_str.trim().parse::<usize>() {
                    Ok(ind) => {
                        if ind < 0 || ind > tasks.len() {
                            println!("Invalid Number out of bounds");
                        }
                        tasks[ind].done = true;
                        save_tasks(&tasks);
                        println!("Task marked as done.");
                    }
                    Err(_) => {
                        println!("Invalid number");
                    }
                }
            }
            "exit" => {
                println!("Exiting");
                break;
            }
            _ => {
                println!("Invalid Operation")
            }
        }
    }
}
