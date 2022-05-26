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
    
    let mut indeces_stts = Vec::new();
    words_stts(&dictionary, &words, &mut indeces_stts);
    
    for (i, status) in indeces_stts.iter().enumerate() {
        if *status == 1 {
            print!("{} ", format!("{}", words[i]).green());
        } else {
            print!("{} ", format!("{}", words[i]).red());
        }
        
    } 
    println!();
    
}

fn get_words (s: &str, vec: &mut Vec<String>) {
    let text = s.as_bytes();

    let mut prev_slice: usize = 0;

    for (i, &item) in text.iter().enumerate() {
        match item {
            b' ' | b'\n' | b',' | b'.' | b';' | b':' | b'"' | b'!' | b'?' | b'-' => {                
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

fn words_stts (dic_words: &Vec<String>, vec_words: &Vec<String>, vec_status: &mut Vec<i32>){
    
    for word in vec_words.iter() {
        let returned_from_find = dic_words.iter().find(|&string| string == word);
        match returned_from_find {
            Option::Some(_result) => vec_status.push(1),
            Option::None => vec_status.push(0),
        };
    }
}
