use std::io;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Default)]
 pub struct Letterconstraint{
    pub min_count : u16,
    pub cap : Option<u16>
}

    fn main(){
        let mut frequecy_map = HashMap::<char,Letterconstraint>::new();
        let  x = ip_word();
        let  y = feedback();
    let possible_words = read_file("allowed_words.txt")
    .expect("File not found");
    println!("total possible words {}", possible_words.len());
    let first_pass_vec = filter(&x,&y,possible_words);
    println!("length {}", first_pass_vec.len());
    frequecy_map_call(&mut frequecy_map,&x,&y);
}

fn ip_word() -> String {
    println!("five letter word");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed input");
    x
}

fn feedback() -> Vec<u16>{
    println!("Enter feedback");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed input");
    let code : Vec <u16> = x.trim().chars().map(|c| c.to_digit(10).expect("failed") as u16).collect();
    code      
}

fn read_file(file_path : &str)-> Result<Vec<String>,io::Error>{
    let possible_words = fs::read_to_string(file_path)?;
    let mut a : Vec<String> = Vec::new();
    for word in possible_words.lines(){
        a.push(word.to_string());
    }
    Ok(a)
}

fn filter(guess : &str, feedback : &Vec<u16>, possible_words : Vec<String>) -> Vec<String>{
    let mut filtered_words = Vec::new();
    for words in possible_words{
        if is_valid(&guess,&feedback,&words){
            filtered_words.push(words);
        }
    }
    filtered_words
}

fn is_valid(guess : &str, feedback : &[u16], word : &str ) -> bool{
    let word_chars : Vec<char> = word.chars().collect();
    let guess_chars : Vec<char> = guess.chars().collect();
    for i in 0..5{
        if feedback[i] == 2{
            if word_chars[i] != guess_chars[i]{
                return false;
            }
        }
    }
    for i in 0..5{
        if feedback[i] == 0{
            if word_chars.contains(&guess_chars[i]){
                return false;
            }
        }
    }
    for i in 0..5{
        if feedback[i] == 1{
            if !word_chars.contains(&guess_chars[i]) || word_chars[i] == guess_chars[i]{
                return false
            }
        }
    }
    true
}

fn frequecy_map_call(frequecy_map : &mut HashMap<char,Letterconstraint>, guess : &str, feedback : &[u16]){
    let guess_chars: Vec<char> = guess.chars().collect();
    for i in 0..5{
        let ch = guess_chars[i];
        let status = feedback[i];
        let constraint = frequecy_map.entry(ch).or_insert(Letterconstraint::default());
        let mut valid_instances = 0;
        // for every letter check how many valid instances of that letter exist
        for idx in 0..5{
            if guess_chars[idx] == ch && (feedback[idx] == 1 || feedback[idx]==2){
                valid_instances += 1;
            }   
        }
        // number of valid instances is equal to the minimum occurences in secret word
        if valid_instances > constraint.min_count {
            constraint.min_count = valid_instances;
        }
        // for each letter in the word if we find a grey current value of valid_instance is the max cap
        if status == 0 {
            constraint.cap = Some(valid_instances)
        }
    } 
    println!("{:?}",frequecy_map)   
}







