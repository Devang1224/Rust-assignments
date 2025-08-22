// 1) store all the file names in an array by using read_dir
// 2) use Arc + Mutex to store rx that comes from mpsc::channel()
// 3) run a loop till the no. of threads 
// 4) clone the rx by using Arc::clone(rx);
// 5) open the file 
// 6) spawn the threads and assign it to a handle variable , lock the rx and recv the items in a while loop
// 7) create a new bufReader using that file and the required buffer size for ex [0u8; 4096] 4kb of buffer
// 8) create a new hasher  Sha256::new()
// 9) use loop till the read bytes becomes zero ( let bytes_read = reader.read(&mut buffer) ) and hash the data
// 10) get the hashed result using hasher.finalize()
// 11) create a folder using create_dir_all and a file to store the hashed result
// 12) at the ending of the for loop push the handle into a Handles vector that stores all the spawned threads JoinHandle
// 13) after the for loop join all the handles by using a loop and join() function to finish the threads


use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read,Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use sha2::{Sha256, Sha512, Digest};
use std::io::{BufReader};
use indicatif::{ProgressBar, ProgressStyle};


fn main() {
    let (tx, rx) = mpsc::channel();
    let THREADS = 4;
    let mut all_files: Vec<String> = Vec::new();
    let mut handles = Vec::new();
    let rx = Arc::new(Mutex::new(rx));

    match fs::read_dir("./files") {
        Ok(entries) => {
            for item in entries {
                match item {
                    Ok(entry) => {
                        all_files.push(entry.file_name().to_string_lossy().to_string());
                        // tries to convert the filename into a valid UTF-8 string, but if the underlying bytes are not valid UTF-8, it replaces invalid parts with the Unicode replacement character � (U+FFFD).
                    }
                    Err(e) => {
                        println!("Error reading the file: {}", e);
                    }
                }
            }
        }
        Err(err) => {
            println!("Unable to read files, Error: {}", err);
            return;
        }
    }

    let pb = Arc::new(ProgressBar::new(all_files.len() as u64)) ;
        pb.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("##-")
        );

    for file in all_files.iter() {
        tx.send(file.to_owned()).unwrap();
    }
    drop(tx);



    for t in 0..THREADS {
        let rx = Arc::clone(&rx);
        let pb = Arc::clone(&pb);
        let thread_id = t;
       let handle = thread::spawn(move || {
            while let Ok(file_path) = rx.lock().unwrap().recv() {
                println!("Thread {thread_id} got file: {file_path}");
                let file = File::open(format!("./files/{file_path}")).expect("Unable to open file");
                let mut reader = BufReader::new(file);
                let mut buffer = [0u8;4096]; // reading 4kb of data 
                let mut hasher = Sha256::new();
                loop{
                    let bytes_read = reader.read(&mut buffer).expect("Error occured while reading");
                    if bytes_read == 0 {
                        break;
                    }
                    hasher.update(&buffer[..bytes_read]);
                }
                let result = hasher.finalize();
                fs::create_dir_all("./hashed_files").expect("Unable to create hashed_files directory");
                let mut out_file = File::create(format!("./hashed_files/{file_path}.txt")).expect("Unable to create output file");
                out_file.write_all(&result).unwrap();
                pb.inc(1);
               
            }
        });
        handles.push(handle);
    }

    for handle in handles {
       handle.join().unwrap(); // Waits for the associated thread to finish.
   }

   pb.finish_with_message("All files hashed!");

}
