use std::{error::Error, fs::File,fs::OpenOptions, io,io::Write, process};
use csv::ReaderBuilder;
use serde::{Deserialize,Serialize};
use serde_json::{Map,Value};




fn main() {
    let mut filePath = String::new();
    println!("Enter the path of csv file: ");

    io::stdin().read_line(&mut filePath).expect("User input is required");
    let filePath = filePath.trim();
    if filePath.is_empty(){
        println!("User input is required");
        return;
    }
   let file = File::open(filePath).expect("Invalid file path");

   
   let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
   let headers = rdr.headers().expect("An error occurred").clone();
   let mut json_rows:Vec<Value> = Vec::new();
    for result in rdr.records() {
       
        match result {
            Ok(res)=>{
                let mut object = Map::new();
                for (h,v) in headers.iter().zip(res.iter()){
                    object.insert(h.to_string(),Value::String(v.to_string())); // Value is the enum from serde_json, representing any valid JSON tyype
                }
                json_rows.push(Value::Object(object));
            }
            Err(err)=>println!("Error occured: {}",err)
        }
    }

    let json_output = serde_json::to_string(&json_rows).expect("Error occurred");
    let mut output_file = OpenOptions::new().create(true).truncate(true).write(true).open("./output_json.json").expect("Unable to opent the file");
     output_file.write_all(&json_output.as_bytes()).expect("Unable to write the file at this moment");
     
}
 