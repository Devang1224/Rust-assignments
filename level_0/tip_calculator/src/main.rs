use std::io;

fn main() {
  let mut userInput1 = String::from("");
  let mut userInput2 = String::from("");
  
  println!("Enter the bill amount $: ");

  io::stdin().read_line(&mut userInput1).expect("User input is required");
  if userInput1.trim().is_empty(){
    panic!("User input is required");
  }
  let amount:f64 = userInput1.trim().parse().expect("Expected a number");

  println!("Enter the tip amount (%): ");

  io::stdin().read_line(&mut userInput2).expect("User input is required");
  if userInput2.trim().is_empty(){
    panic!("User input is required");
  }
  let tip:f64 = userInput2.trim().parse().expect("Expected a number");
  
   let totalAmount = amount + (amount*(tip/100.0));
   println!("Total amount: {}$",format!("{:2}",totalAmount));

}
