use std::fs;
use std::io;

fn main(){
    let possible_words = read_file("allowed_words.txt")
    .expect("File not found");
    for word in possible_words {
        println!("{}",word);
    }

}

// fn filter(a : String, b : Vec<u16>) -> Vec<String>{
//     // remove the greys
//     // if green is in correct position add the word to the vector and return


// }

fn read_file(file_path : &str)-> Result<Vec<String>,io::Error>{
    let possible_words = fs::read_to_string(file_path)?;
    let mut a : Vec<String> = Vec::new();
    for word in possible_words.lines(){
        a.push(word.to_string());
    }
    Ok(a)
}