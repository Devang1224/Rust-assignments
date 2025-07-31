use std::io;



fn main() {
    let mut userInput = String::from("");
    let mut userInput2 =  String::from("");




    println!("!!!!!!!!! Convert temperature !!!!!!!!!!");
    
    println!("Enter a temperature: ");
    let value1 = io::stdin().read_line(&mut userInput).expect("Need a user input");
    if userInput.trim().is_empty() {
        panic!("Need a temperature to convert");
    }
    let temperature:f64 = userInput.trim().parse().expect("Invalid value");

    println!("Enter the current unit of temperature:  ");
    let value3 = io::stdin().read_line(&mut userInput2).expect("Need a user input");
    if userInput2.trim().is_empty() {
        panic!("Need a unit");
    }


    // let value2 = io::stdin().read_line(&mut convertTo).expect("Need a user input");
    // if convertTo.trim().is_empty(){
    //     panic!("Need a value (C or F)");
    // }

    let ans = match userInput2.trim() {
        "C"|"c" => (temperature*(1.8))+32.0,
        "F"|"f" => (temperature-32.0)/1.8,
        _ => panic!("Invalid Unit")
    };

      let convertedTempType = if userInput2.trim() == "C" || userInput2.trim() == "c"{
        "F"
      }else {
        "C"
      };
      println!("converted temperature: {}{}",ans,convertedTempType);
      
}
