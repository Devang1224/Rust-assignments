use std::io;

fn is_anagram(s1:&str,s2:&str)->bool{

    if s1.len()!=s2.len() {
        return false;
    }
  
  let mut v1:Vec<char> = s1.chars().collect();
  let mut v2:Vec<char> = s2.chars().collect();
   v1.sort_unstable();
   v2.sort_unstable();

    return v1==v2;

}

fn main() {
    let mut userInput2 = String::new();
    let mut userInput1 = String::new();

 println!("Enter first string: ");

  io::stdin().read_line(&mut userInput1).expect("User input is required");
  if userInput1.trim().is_empty() {
    panic!("User input is required");
  }

  println!("Enter second string: ");
  io::stdin().read_line(&mut userInput2).expect("User input is required");
  if userInput2.trim().is_empty() {
    panic!("User input is required");
  }
  

println!("Anagram Status: {}",is_anagram(&userInput1, &userInput2));
  

}
