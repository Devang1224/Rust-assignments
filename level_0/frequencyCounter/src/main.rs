use std::io;
use std::collections::HashMap;

fn main() {
  let mut userInput = String::new();
  let mut frequency:HashMap<char,u32> = HashMap::new();

  println!("Enter a String: ");
  io::stdin().read_line(&mut userInput).expect("User input is required");
  if userInput.trim().is_empty(){
    panic!("User input is required");
  }
  userInput = userInput.trim().to_string();

  for i in userInput.chars(){
   
   // can be optimized by using this :  *frequency.entry(ch).or_insert(0) += 1;
    match frequency.get_mut(&i) {
        Some(item)=>{
            *item+=1;
        },
        None=>{
            frequency.insert(i, 1);
        }
    }
  }

  println!("Frequency: ");
  for (item,value) in frequency.iter() {
      println!("{} : {}",item,value);
    }



}
