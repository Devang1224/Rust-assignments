// Problem statement; 
// - Take two numbers and an operator (+, -, *, /) from user input.
// - Perform the operation and print the result.
// - Practice `match`, functions, and error handling.

use std::io;



// fn checkValidOperator(op:&str)->Result<(),String>{

//   match op {
//     "+"|"-"|"*"|"/"|"%" => Ok(()),
//     _ => Err(format!("Invalid Operator : {}",op)),

//   }
// }

fn performAction(num1:&f64,num2:&f64,userAction:&str)->f64{

    match userAction {
       "+"=>*num1 + *num2,
       "-"=>*num1 - *num2,
       "/"=>*num1 - *num2,
       "%"=>*num1 % *num2,
       _=>panic!("Invalid Operator in performAction!")
    }

}

fn main() {
 
let mut userInput1 = String::from(""); 
let mut userInput2 = String::from("");
let mut userAction = String::from("");

  println!("Enter first number: ");
// you can use text_io crate to read input from the user
 io::stdin().read_line(&mut userInput1).expect("Error: User input is required");
 if userInput1.trim().is_empty() {
    panic!("Error: User input cannot be empty");
 }
  
  println!("Enter second number: ");
  io::stdin().read_line(&mut userInput2).expect("Error: User input is required");
  if userInput2.trim().is_empty() {
    panic!("Error: User input cannot be empty");
 }

  println!("Enter an Operator (+, -, *, /, %): ");
  io::stdin().read_line(&mut userAction).expect("Error: user action is required");
   if userAction.trim().is_empty() {
    panic!("Error: Action cannot be empty");
  }
//   match checkValidOperator(&userAction.trim()){
//     Ok(())=>{},
//     Err(err)=>{
//         panic!("{}",err)
//     }
//   }
   

 let num1:f64 = userInput1.trim().parse().expect("Expected a number");
 let num2:f64 = userInput2.trim().parse().expect("Expected a number");

 let result:f64 = performAction(&num1,&num1,&userAction.trim());

 println!("{} {} {} = {}",num1,userAction,num2,result);
  

  
}


