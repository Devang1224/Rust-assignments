use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

use bcrypt::{DEFAULT_COST, hash, verify};


#[derive(Serialize, Deserialize)]


struct User {
    username: String,
    password: String,
}

trait FileExistence {
    fn check_file_existence() -> bool;
}



impl User {
    fn new(username: String, password: String) -> User {
        let hashed_password = hash(password,DEFAULT_COST).expect("Failed to hash the password");
        User {
            username: username,
            password: hashed_password,
        }
    }

    fn load_users() -> HashMap<String, String> {
        if !User::check_file_existence() {
            return HashMap::new();
        }
        let mut file = File::open("./users.json").expect("Unable to open the file at this moment");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new())
    }

    fn save_users(users: &HashMap<String, String>) {
        let fromatted_data =
            serde_json::to_string_pretty(users).expect("Unable to convert the data into json");
        let mut file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .read(true)
            .create(true)
            .open("./users.json")
        {
            // truncate is used to clear all the data from the file before saving the new data
            Ok(file) => file,
            Err(_) => {
                println!("Unable to open the file at this moment. Try again later");
                return;
            }
        };
        file.write_all(fromatted_data.as_bytes()).expect("Unabel to save the data at this moment");

        println!("Registerd successfully");

    }
}

impl FileExistence for User {
    fn check_file_existence() -> bool {
        let file_exists = Path::new("./users.json").exists();
        if file_exists {
            return true;
        }
        return false;
    }
}

fn main() {
    let mut users: HashMap<String, String> = User::load_users();
    loop {
        let mut operation = String::new();
        println!("Enter an operation to perform: (login,register,exit)");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut operation)
            .expect("User input is required");
        let operation = operation.trim();
        if operation.is_empty() {
            println!("User input is required");
        }

        match operation {
            "login" => {
                let mut username = String::new();
                let mut password = String::new();
                println!("Enter useraname: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Username is required");
                let username = username.trim();
                if username.is_empty() {
                    println!("username cannot be empty");
                    continue;
                }
                if !users.contains_key(username) {
                    println!("username not found. If you dont have any account the please register.");
                    continue;
                }
                println!("Enter password: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut password)
                    .expect("user input is reqruired");
                let password = password.trim();
                if password.is_empty() {
                    println!("password cannot be empty");
                    continue;
                }

                let correct_password = verify(password, &users[username]).unwrap_or(false);
                if !correct_password{
                    println!("Incorrect password");
                    continue;
                }

                println!("Logged in successfully");

            }
            "register" => {
                let mut username = String::new();
                let mut password = String::new();
                println!("Enter username: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Username is required");
                let username = username.trim();
                if username.is_empty() {
                    println!("username cannot be empty");
                    continue;
                }
                println!("Enter password: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut password)
                    .expect("user input is reqruired");
                let password= password.trim();
                if password.is_empty() {
                    println!("password cannot be empty");
                    continue;
                }

                let new_user = User::new(username.to_owned(), password.to_owned());
                // add user to users hashmap
                users.insert(new_user.username, new_user.password);
                User::save_users(&users);
            }
            "exit" => {
                println!("Exiting");
                break;
            }
            _ => {
                println!("Invalid operation");
            }
        }
    }
}
