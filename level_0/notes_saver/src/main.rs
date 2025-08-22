use std::io;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let fileExists = Path::new("./notes.txt").exists();
    let mut userInput = String::new();

    if !fileExists { 
        match File::create("notes.txt") {
            Ok(_)=>{},
            Err(_)=>{
                println!("Unable to create a file");
                return;
            }
        }        
    }
    println!("Enter a new note:");
    io::stdin().read_line(&mut userInput).expect("User input is required");
    userInput = userInput.trim().to_string();
    if userInput.is_empty(){
        println!("User input is required");
        return;
    }

   let mut file = OpenOptions::new().append(true).open("./notes.txt").unwrap(); 
   file.write_all(userInput.as_bytes());

    println!("Updated successfully");

   

}
