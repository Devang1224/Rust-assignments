use std::io;

fn main() {

 let mut userInput1 = String::from(""); 
 let mut userInput2 = String::from("");
 let mut ans:Vec<String> = Vec::new();

 println!("Enter a number: ");
 let num1 = io::stdin().read_line(&mut userInput1).expect("Need a number to proceed");
 if userInput1.trim().is_empty(){
    panic!("Need a number to proceed");
 }
 let mut firstNum:i32 = userInput1.trim().parse().expect("Expected a number");

 println!("Enter a number: ");
 io::stdin().read_line(&mut userInput2).expect("Need a number to proceed");
 if userInput2.trim().is_empty(){
    panic!("Need a number to proceed");
 }
 let mut secondNum:i32 = userInput2.trim().parse().expect("Expected a number");

 let mut a = 0;
 let mut b = 0;
 
 for i in 1..101 {
     a = a+1;
     b = b+1;
     let mut flag = false;

    if a == firstNum && b == secondNum {
        ans.push(String::from("FizzBuzz"));
        flag=true;
        a=0;b=0;
    }
    if a == firstNum {
        ans.push(String::from("Fizz"));
        flag=true;
        a=0;
    }
    if b == secondNum {
        ans.push(String::from("Buzz"));
        flag=true;
        b=0;
    }
    if !flag {
        ans.push(String::from(i.to_string()))
    }
    
 }
 
  for i in ans.iter(){
    println!("{}",i);
  }


}
