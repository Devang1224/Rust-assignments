use std::io;

fn main() {
   let mut userInput = String::new();
   println!("Enter a string: ");
   io::stdin().read_line(&mut userInput).expect("User input is required");
   if userInput.trim().is_empty(){
    panic!("User input is required");
   }
  
  let mut ans = String::new();

  for char in userInput.chars(){
    let encoded_char = if char.is_ascii_lowercase() {
            (((char as u8 - b'a' + 3) % 26) + b'a') as char
        } else if char.is_ascii_uppercase() {
            (((char as u8 - b'A' + 3) % 26) + b'A') as char
        } else {
            char // leave non-letters unchanged (space, punctuation, etc.)
        };
    ans.push_str(&encoded_char.to_string());
  }

  println!("{}",ans);



}
