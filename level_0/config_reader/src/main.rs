use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;


fn main() {

  let mut map:HashMap<String,String> = HashMap::new();

  let mut file  = match File::open("config.txt") {
    Ok(file)=>file,
    Err(_)=>{
        println!("File not found");
        return;
    }
  };
  
  let mut buffer = vec![0;1000000]; // 1MB
  let bytes_read = file.read(&mut buffer).expect("Unexpected Error");

  let content = String::from_utf8_lossy(&buffer[..bytes_read]);
  
  //  println!("{:?}",content.split("="));
  

   for item in content.split_whitespace(){
      let values:Vec<&str> = item.split('=').collect();
       let key = values[0].to_string();
       let value = values[1].to_string();
       map.insert(key,value);
   }


}
