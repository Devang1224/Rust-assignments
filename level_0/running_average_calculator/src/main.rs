use std::io;

fn main() {
    let mut userInput = String::new();

    println!("Enter a number (>2)");

    io::stdin()
        .read_line(&mut userInput)
        .expect("User input is required");
    userInput = userInput.trim().to_string();
    if userInput.is_empty() {
        println!("User input is required");
        return;
    }

    let num = match userInput.parse::<u32>() {
        Ok(item) => item,
        Err(_) => {
            println!("Invalid Number");
            return;
        }
    };
    if num < 2 {
        println!("Enter a number greater than 2");
        return;
    }

    let mut primeNum = vec![0; (num + 1) as usize];

    for i in 2..num {
        if primeNum[i as usize] == 0 {
            let mut j: u32 = i*i;
            while j <= num {
                //  you can convert one type to another by using try_into trait
                // if let Ok(index) = num.try_into() {
                //     if let Some(x) = v.get_mut(index) {
                //         *x += 1;
                //     } else {
                //         println!("Index out of bounds");
                //     }
                // } else {
                //     println!("Conversion failed");
                // }

                primeNum[j as usize] = 1;
                j += i;
            }
        }
    }
    println!("Prime numbers: ");

    for (ind, value) in primeNum.iter().enumerate() {
        if *value == 0 {
            println!("{}", ind);
        }
    }
}
