use std::collections::HashMap;
use std::io;

fn main() {
    let mut phoneBook: HashMap<String, u32> = HashMap::new();

    loop {
        let mut operation = String::new();
        println!("Enter an operation: 
        -search,
        -insert,
        -update,
        -delete,
        -list,
        -exit");

        io::stdin()
            .read_line(&mut operation)
            .expect("User input is required");
        if operation.trim().is_empty() {
            println!("User input is required");
            continue;
        }

        match operation.trim() {
            "search" => {
                let mut searchInput = String::new();
                println!("Search by name or phone no.");
                io::stdin()
                    .read_line(&mut searchInput)
                    .expect("User input is required");
                if searchInput.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                let parsedNum = searchInput.trim().parse::<u32>();
                if let Ok(number) = parsedNum {
                    let num = phoneBook.iter().find(|(_, &v)| v == number);
                    match num {
                        Some((name, number)) => println!("Name: {} | Number: {}", name, number),
                        None => println!("Not found"),
                    }
                } else {
                    match phoneBook.get(searchInput.trim()) {
                        Some(item) => {
                            println!("Number: {}", item);
                        }
                        None => println!("Not found"),
                    }
                }
            }
            "insert" => {
                let mut name = String::new();
                let mut phoneNum = String::new();

                println!("Enter name: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("User input is required");
                if name.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                println!("Enter phone number:  ");
                io::stdin()
                    .read_line(&mut phoneNum)
                    .expect("User input is required");
                if phoneNum.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }

                let parsedNum = match phoneNum.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Number");
                        continue;
                    }
                };

                let userExists = phoneBook.contains_key(name.trim());
                if userExists {
                    println!("User already exists");
                    continue;
                }
                match phoneBook.insert(name.trim().to_string(), parsedNum) {
                    Some(_) => println!("Unexpected overwrite occurred."),
                    None => println!("Data added successfully"), // HashMap::insert will return none if the data is inserted and not overWritten
                }
            }
            "update" => {
                let mut name = String::new();
                let mut phoneNum = String::new();

                println!("Enter name: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("User input is required");
                if name.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                println!("Enter new phone number to update:  ");
                io::stdin()
                    .read_line(&mut phoneNum)
                    .expect("User input is required");
                if phoneNum.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }

                let parsedNum = match phoneNum.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Number");
                        continue;
                    }
                };

                match phoneBook.get_mut(name.trim()) {
                    Some(value) => {
                        *value = parsedNum;
                    }
                    None => println!("Not found"),
                }
            }
            "delete" => {
                let mut name = String::new();
                println!("Enter name: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("User input is required");
                if name.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }

                let userExists = phoneBook.contains_key(name.trim());
                if !userExists {
                    println!("Not found");
                    continue;
                }

                match phoneBook.remove(name.trim()) {
                    Some(_)=>println!("Removed successfully"),
                    None=>println!("Not found")
                }

            }
            "list" => {
                if phoneBook.len() == 0 {
                    println!("List is empty");
                    continue;
                }
                phoneBook
                    .iter()
                    .for_each(|(name, value)| println!("Name: {}, PhoneNo: {}", name, value));
            }
            "exit"=>{
                println!("Exitted successfully");
                break;
            }
            _ => println!("Invalid Operation"),
        }
    }
}
