use std::collections::HashMap;
use std::io;

fn main() {
    let mut cart: HashMap<String, f64> = HashMap::new();

    loop {
        let mut operation = String::new();

        println!("Enter an operation to perform: add,remove,list, exit");

        io::stdin()
            .read_line(&mut operation)
            .expect("User input is required");
        if operation.trim().is_empty() {
            println!("User input is required ");
            continue;
        }

        match operation.trim() {
            "add" => {
                let mut cartItem = String::new();
                let mut itemCost = String::new();
                println!("Enter item name: ");
                io::stdin()
                    .read_line(&mut cartItem)
                    .expect("User input is required");
                if cartItem.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                println!("Enter item's cost: ");
                io::stdin()
                    .read_line(&mut itemCost)
                    .expect("User input is required");
                if itemCost.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                let cost: f64 = match itemCost.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                cart.insert(cartItem.trim().to_string(),cost);
                println!("Item inserted successfully");
            }
            "remove" => {
                let mut cartItem = String::new();
                println!("Enter item name: ");
                io::stdin()
                    .read_line(&mut cartItem)
                    .expect("User input is required");
                if cartItem.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                if cart.remove(cartItem.trim()).is_some() {
                    println!("Removed successfully");
                }else {
                    println!("Item not found");
                }
                
            }
            "list" => {
                let itr = cart.iter();
                for (name,value) in itr{
                    println!("Item: {}, Cost: {}",name,value);
                }
            }
            "exit"=>{
                println!("Exitted successfully");
                break;
            }
            _ => println!("Invalid operation"),
        }
    }
}
