use std::env;
use std::fs;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    //  Verify if arguments is being passed
    if args.len() < 3 {
        println!("Missing argument!!!");
        return;
    }

    //  Load and split words of a file
    let filename = String::from(&args[1]);
    let contents = fs::read_to_string(filename).expect("Error at reading file!!!");
    let mut words: Vec<String> = Vec::new(); 
    get_words(&contents, &mut words);
    
    //  Load and split each word of the correspondent dictionary to dictionary vector
    let dic_to_load = String::from(&args[2]);
    let dictionary: Vec<String> = load_dictionary(dic_to_load).unwrap();
    
//    for (i, item) in words.iter().enumerate() {
//        println!("{} - {}", i, format!("{}", item).bold().blue());
//    }
    let indeces_stts = words_stts(&dictionary, &words);
    for (index, word) in words.iter().enumerate() {
        match indeces_stts[index] {
            1 => println!("{}", format!("{}", word).bold().blue()),
            _ => println!("{}", format!("{}", word).bold().red()),
        }
    }
}

fn get_words (s: &str, vec: &mut Vec<String>) {
    let text = s.as_bytes();

    let mut prev_slice: usize = 0;

    for (i, &item) in text.iter().enumerate() {
        match item {
            b' ' | b'\n' | b',' | b'.' | b';' | b':' | b'"' => {                
                vec.push(s[prev_slice..i].to_string().to_lowercase());
                prev_slice = i + 1;
            } 
            _ => continue
        }
    }
}

fn load_dictionary (dic: String) -> Option<Vec<String>> {
    match dic.as_str() {
        "br" => {
            let path = "../dics/br-utf8.txt";
            let dictionary = fs::read_to_string(path).expect("Error at reading dictionary");
            let mut words_splitted: Vec<String> = Vec::new();
            get_words(&dictionary, &mut words_splitted);
            
            Some(words_splitted)

        }
        _ => {
            println!("Invalid dictionary!!!");
            
            None
        }
    }
}

fn words_stts (dic_words: &Vec<String>, vec_words: &Vec<String>) -> Vec<i8>{
    let mut word_stts: Vec<i8> = Vec::new();
    
    for word_in_text in vec_words.iter() {
        for i in 0..dic_words.len() {
            if word_in_text.to_string() == dic_words[i] {
                word_stts.push(1);
                break;
            }
            else if i == dic_words.len() - 1 {
                word_stts.push(-1);
            }
            else {
                continue;
            }
        }
    } 

    return word_stts;
}
