use std::{collections::HashMap, io};

fn main() {
    let mut userInput = String::new();
    let mut wordFreqency: HashMap<String, u32> = HashMap::new();
    let mut longestWord = String::new();

    println!("Enter a sentence: ");
    io::stdin()
        .read_line(&mut userInput)
        .expect("User input is required");
    if userInput.trim().is_empty() {
        panic!("User input is required");
    }

    let words: Vec<&str> = userInput.trim().split_whitespace().collect();

    for word in words.iter() {
        if word.len() > longestWord.len() {
            longestWord = word.to_string();
        }

        let frequency = wordFreqency.get(*word);
        match frequency {
            Some(item) => {
                wordFreqency.insert(word.to_string(), item + 1);  // rust is automatically dereferencing here 
            }
            None => {
                wordFreqency.insert(word.to_string(), 1);
            }
        }
    }
    // or you can direclty do this : *map.entry(key).or_insert(0) += 1;  here * is used to dereference the &mut u32 to get the u32

    println!(
        "Total Words: {}, Longest Word: {}",
        words.len(),
        longestWord
    );
    println!{"Frequency Map: {:?}",wordFreqency}
}

