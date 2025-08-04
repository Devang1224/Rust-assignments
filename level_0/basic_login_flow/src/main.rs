use std::collections::HashMap;
use std::io;

struct User {
    name: String,
    password: String,
}

impl User {
    fn validate(&self, password: &str) -> bool {

        if self.password == password {
            return true;
        }
        return false;
    }
}

fn readInput() -> Option<String> {
    let mut userInput = String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("User input is required");
    if userInput.trim().is_empty() {
        println!("User input is required");
        return None;
    }
    return Some(userInput.trim().to_string());
}

fn main() {
    let mut users: HashMap<String, User> = HashMap::new();

    loop {
        println!("Enter any operation register, login, exit");

        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("User input is required");
        if operation.trim().is_empty() {
            println!("user input is required");
        }

        match operation.trim() {
            "register" => {
                println!("Enter username: ");
                let userName = match readInput() {
                    Some(user) => user,
                    None => {
                        println!("Invalid String");
                        continue;
                    }
                };
                if users.contains_key(&userName) {
                    println!("User already exists");
                    continue;
                }
                println!("Enter password: ");
                let password = match readInput() {
                    Some(userPass) => userPass,
                    None => {
                        println!("Invalid String");
                        continue;
                    }
                };

                let newUser = User {
                    name: userName.clone(),
                    password: password,
                };
                users.insert(userName, newUser);
                println!("User registered successfully");
            }
            "login" => {
                println!("Enter username: ");
                let userName = match readInput() {
                    Some(user) => user,
                    None => {
                        println!("Invalid String");
                        continue;
                    }
                };
                println!("Enter password: ");
                let password = match readInput() {
                    Some(userPass) => userPass,
                    None => {
                        println!("Invalid String");
                        continue;
                    }
                };
                let existingUser = users.get(&userName);

                let isCorrectPassword = match existingUser {
                    Some(user) => {
                        user.validate(&password)
                    }
                    None => {
                        println!("User not found");
                        continue;
                    }
                };

                if !isCorrectPassword {
                    println!("Incorrect password");
                } else {
                    println!("User loggedin successfully");
                }
            }
            "exit" => {
                println!("Exited successfully");
                break;
            }
            _ => println!("Invalid operation"),
        }
    }
}
