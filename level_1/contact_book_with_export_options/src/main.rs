use std::{io,io::Write,fs::OpenOptions};
use serde::{Deserialize};
use serde_json;
use csv::Writer;
use chrono::{DateTime, Local};

#[derive(Debug, PartialEq)]
enum Export_method {
    CSV,
    JSON
}

#[derive(serde::Serialize, Deserialize)]
struct User {
    name:String,
    phone:String,
    email:String
}
impl User {
    fn new(name:String,phone:String,email:String)->User{
        User {
            name:name,
            phone:phone,
            email:email
        }
    }
}

fn ask_for_input(key:String,custom_prompt:Option<&str>)->Result<String,String>{
    let mut input = String::new();
    if let Some(prompt) = custom_prompt{
        println!("{}",prompt);
    }else{
        println!("Enter user's {}: ",key);
    }
    io::stdout().flush().unwrap();

    if let Err(_) = io::stdin().read_line(&mut input){
        return Err("User input is required".to_string());
    }

    let user_input = input.trim().to_string();
    if user_input.is_empty(){
        Err("Input cannot be empty".to_string())
    }else{
      Ok(user_input)
    }


}

fn export_to_csv(user_details:&Vec<User>){
  if user_details.is_empty(){
    println!("List should have atleast one row");
    return;
   }
   
   let path: String = format!("./{}.csv", Local::now().format("%Y-%m-%d_%H-%M-%S"));
    
   let mut wtr = match csv::Writer::from_path(&path) {
    Ok(writer)=>writer,
    Err(err)=>{
         println!("Error creating CSV file: {}", err);
            return;
    }
   };
  
   for user in user_details {
      if let Err(err) = wtr.serialize(user){
          println!("Error writing record: {}", err);
            return;
      }
   }
     if let Err(err) = wtr.flush() {
        println!("Error flushing CSV: {}", err);
        return;
    }

    println!("Exported to CSV: {}",path);


  
}

fn export_to_json(user_details:&Vec<User>){
   if user_details.is_empty(){
    println!("List should have atleast one row");
    return;
   }
   
  let json_data = match serde_json::to_string_pretty(user_details) {
    Ok(data)=>data,
    Err(err)=>{
        println!("An unexpected error comes up. Try again later");
        println!("{}",err);
        return;
        
    }
  };
   
   let path: String = format!("./{}.json", Local::now().format("%Y-%m-%d_%H-%M-%S"));
   

  let mut file = OpenOptions::new().write(true).create(true).open(path.clone()).expect("Failed to create a json file");
  file.write_all(json_data.as_bytes()).expect("Unable to create a json file at this moment");
  println!("Exported to JSON: {}",path);

}

fn main() {
  
  let mut user_details:Vec<User> = Vec::new();

  loop{
    let mut operation = String::new();
     println!("Enter an operation: 
     1) create
     2) delete
     3) update
     4) read
     5) list
     6) export 
     7) exit
     ");
     io::stdout().flush().unwrap();
  
    io::stdin().read_line(&mut operation).expect("User input is required");
    if operation.trim().is_empty(){
        println!("User input is required");
        continue;
    }

    match operation.trim(){
        "create"=>{
                let user_name = match ask_for_input("name".to_string(),None){
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
                let user_phone = match ask_for_input("phone".to_string(),None){
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
                let user_email = match ask_for_input("email".to_string(),None){
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
                let new_user = User::new(user_name,user_phone,user_email);
                user_details.push(new_user);
                println!("User created successfully");

                
        }
        "read"=>{
                let user_info = match ask_for_input("name".to_string(),Some("Search by name, phone, or email")) {
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };

            let matches:Vec<&User> = user_details.iter().filter(|val| val.name == user_info || val.email == user_info || val.phone == user_info).collect();
            
            if matches.is_empty() {
                println!{"-----NO RECORDS FOUND-----"};
            }else {
                println!("Name | Email | Phone ");
                for val in &matches{
                    println!("{} | {} | {} ",val.name,val.email,val.phone);
                }
            }
        }
        "update"=>{
            let user_input = match ask_for_input("name".to_string(),Some("Search by name,phone, or email")) {
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
            let field_name = match ask_for_input("name".to_string(),Some("Enter field name to update: name,email,phone")) {
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
            let updated_value = match ask_for_input("".to_string(),Some("Enter updated value: ")) {
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };

            match user_details.iter_mut().find(|val| val.name == user_input || val.email == user_input || val.phone == user_input) {
                Some(val)=>{
                   match field_name.as_str(){
                    "name"=> val.name = updated_value.to_owned(),
                    "email"=>val.email = updated_value.to_owned(),
                    "phone"=>val.phone = updated_value.to_owned(),
                    _=>{
                        println!("Invalid field name");
                        continue;
                    }
                   }
                   println!("---- USER UPDATED SUCCESSFULLY ---");
                }
                None=>{
                    println!("---- USER NOT FOUND ----");
                    continue;
               }
            };


        }
        "delete"=>{
            let user_info = match ask_for_input("name".to_string(),Some("Search by name, phone, or email")) {
                    Ok(val)=>val,
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };
              if let Some((index,user)) = user_details.iter().enumerate().find(|(_, val)| val.name == user_info || val.email == user_info || val.phone == user_info) {
                  println!("Name | Email | Phone ");
                  println!("{} | {} | {}",user.name,user.email,user.phone);

                let is_delete:bool = match ask_for_input("".to_string(), Some("Are you sure you want to delete this user: (Y)yes (N)no")) {
                    Ok(val)=>{
                        if val.to_lowercase() == "y" {
                             true
                        }else {
                            false
                        }

                    }
                    Err(_)=>{
                        println!("Invalid Choice");
                        false
                    }
                };
                if !is_delete {
                    println!("Deletion cancelled");
                    continue;
                }

                user_details.swap_remove(index);
                 
                println!("----DELETED SUCCESSFULLY----");

              }else {
                  println!("-----USER NOT FOUND-----");
                  continue;
              }


            
        }
        "export"=>{
            let export_method:Export_method = match ask_for_input("name".to_string(),Some("Export to CSV or JSON: ")) {
                    Ok(val)=>{
                        match val.trim().to_lowercase().as_str(){
                            "csv"=>Export_method::CSV,
                            "json"=>Export_method::JSON,
                            _=>{
                                println!("Invalid choice");
                                continue;
                            }
                        }
                    }
                    Err(err)=>{
                        println!("Error: {}",err);
                        continue;
                    }
                };

                if export_method == Export_method::CSV{
                    export_to_csv(&user_details);
                }else if export_method == Export_method::JSON{
                    export_to_json(&user_details);

                }



        }
        "exit"=>{
            println!("Exiting");
            break;
        }
        "list"=>{
            println!("Name | Email | Phone ");
            for val in user_details.iter(){
                println!("{} | {} | {} ",val.name,val.email,val.phone);
            }
        }
        _=>{
            println!("Invalid operation. Try again");
        }
    }

    
  }

    
}
