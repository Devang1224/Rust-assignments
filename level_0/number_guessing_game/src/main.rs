use std::io;
use rand::rng;


fn askForInput()->u32{
 let mut userInput = String::from("");  
 io::stdin().read_line(&mut userInput).expect("User Input is required");
  if userInput.trim().is_empty(){
    panic!("user input is required");
  }
  let num:u32 = userInput.trim().parse().expect("User input is required");
  return num;
}

fn main() {
    let randomNumber = rand::random_range(1..=20);
    
    println!("Guess the number (1-20): ");
  
  let mut num = askForInput();

  while randomNumber!=num {
      if num<randomNumber {
       println!("Too Low");
      } else if num > randomNumber {
       println!("Too High");
      }

    println!("Guess Again");
    num = askForInput();
   if num == randomNumber {
    println!("correct!!");
   }
  }



}
