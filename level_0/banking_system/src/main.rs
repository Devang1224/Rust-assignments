use std::collections::HashMap;
use std::io;

struct Bank {
    user: String,
    balance: f64,
}

impl Bank {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited succesfully--------");
    }

    fn withDraw(&mut self, amount: f64) -> f64 {
        if amount > self.balance {
            println!("Insufficient balance");
            return 0.0;
        }
        self.balance -= amount;
        println!("Operated succesfully--------");
        return self.balance;
    }

    fn getBalance(&self) -> f64 {
        return self.balance;
    }
    fn register(&mut self, userName: &str) {
        self.user = userName.to_string();
        self.balance = 0.0;
        println!("Registered succesfully--------");
    }
}

fn enterUserName() -> String {
    println!("Enter username: ");
    let mut userName = String::new();
    io::stdin()
        .read_line(&mut userName)
        .expect("User input is required");

    return userName.trim().to_string();
}

fn main() {
    let mut users: HashMap<String, Bank> = HashMap::new();

    loop {
        let mut operation = String::new();

        println!(
            "Enter operation to perform:- 
        Register,
        Deposit,
        Withdraw,
        Balance,
        Exit
        "
        );
        io::stdin()
            .read_line(&mut operation)
            .expect("User input is required");
        if operation.trim().is_empty() {
            println!("User input is required");
            continue;
        }
        if operation.trim() == "Exit" {
            break;
        }

        match operation.trim() {
            "Register" => {
                let userName = enterUserName();
                if userName.is_empty() {
                    println!("User input is required");
                    continue;
                }

               if users.contains_key(&userName){
                  println!("User with this name already exists");
                  continue;
                }

                users.insert(
                    userName.clone(),
                    Bank {
                        user: userName,
                        balance: 0.0,
                    },
                );
            }
            "Deposit" => {
                let userName = enterUserName();
                if userName.is_empty() {
                    println!("User input is required");
                    continue;
                }
                println!("Enter amount: ");

                let mut amountInput = String::new();
                io::stdin()
                    .read_line(&mut amountInput)
                    .expect("User input is required");
                if amountInput.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                let amount: f64 = match amountInput.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number format");
                        continue;
                    }
                };
                if amount < 0.0 {
                    println!("Cannot deposit negative amount");
                    continue;
                }
                match users.get_mut(&userName) {
                    Some(user) => {
                        user.deposit(amount);
                    }
                    None => {
                        println!("User not found");
                        continue;
                    }
                }
            }
            "Withdraw" => {
                let userName = enterUserName();
                if userName.is_empty() {
                    println!("User input is required");
                    continue;
                }
                let mut userInput = String::new();
                println!("Enter amount: ");

                io::stdin()
                    .read_line(&mut userInput)
                    .expect("User input is required");
                if userInput.trim().is_empty() {
                    println!("User input is required");
                    continue;
                }
                let withdrawAmount: f64 = match userInput.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number format");
                        continue;
                    }
                };
                if withdrawAmount < 0.0 {
                    println!("Cannot withdraw negative amount");
                    continue;
                }
                let userStruct = users.get_mut(&userName);
                match userStruct {
                    Some(user) => {
                      let remaining = user.withDraw(withdrawAmount);
                      println!("Remaining Balance: ",remaining);
                    }
                    None => {
                        println!("User not found");
                        continue;
                    }
                }
            }
            "Balance" => {
                let userName = enterUserName();
                if userName.is_empty() {
                    println!("User input is required");
                    continue;
                }
                let userStruct = users.get(&userName);

                match userStruct {
                    Some(user) => {
                        println!("current balance: {}", user.getBalance());
                    }
                    None => {
                        println!("User not found");
                        continue;
                    }
                }
            }
            _ => {
                println!("Invalid operation. Please try again.");
                continue;
            }
        }
    }
}
