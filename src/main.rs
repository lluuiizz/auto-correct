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
    /*for (index, word) in words.iter().enumerate() {
        match indeces_stts[index] {
            1 => println!("{}", format!("{}", word).bold().blue()),
            _ => println!("{}", format!("{}", word).bold().red()),
        }
    }*/
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

fn words_stts (dic_words: &Vec<String>, vec_words: &Vec<String>) -> Vec<u8>{
    let mut word_stts: Vec<u8> = Vec::new();
    let mut init: usize = 0;
    
    for (indx, text_word) in vec_words.iter().enumerate() {
        let text_bytes = text_word.as_bytes();
        let text_first_char = text_bytes[0];
         
        for (i, dic_word) in dic_words.iter().enumerate() {
            let dic_bytes = dic_word.as_bytes();
            let dic_first_char = dic_bytes[0];
            if dic_first_char == text_first_char {
                init = i;
                break;
            }
        }

        let vec_dic = &dic_words[init..];
        for word_in_dic in vec_dic.iter() {
            if word_in_dic == text_word {
                word_stts.push(1);
                break;
            }  
        } 
        init = 0;

    }
    return word_stts;
}
