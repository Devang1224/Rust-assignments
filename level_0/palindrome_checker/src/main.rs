use std::io;
 
fn main() {
    let mut userInput = String::new();
    let mut flag = true;

    println!("Enter a string: ");
    io::stdin().read_line(&mut userInput).expect("User input is required");
    if userInput.trim().is_empty(){
        panic!("User input is required");
    }
   
// nth(i) will take O(n) time it start the iterator from the starting for every single iteration

  let chars: Vec<char> = userInput.trim().chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;
    let mut flag = true;

    while i < j {
        if chars[i] != chars[j] {
            flag = false;
            break;
        }
        i += 1;
        j -= 1;
    }

   println!("{}",flag);
   

}
