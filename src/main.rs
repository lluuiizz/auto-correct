use std::env;
use std::fs;

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
    
    for (i, item) in words.iter().enumerate() {
        println!("{} - {}", i, item);
    }
}

fn get_words (s: &str, vec: &mut Vec<String>) {
    let text = s.as_bytes();

    let mut prev_slice: usize = 0;

    for (i, &item) in text.iter().enumerate() {
        match item {
            b' ' | b'\n' | b',' | b'.' | b';' | b':' => {                
                vec.push(s[prev_slice..i].to_string().to_lowercase());
                prev_slice = i;
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

/*fn pop_trash (vec: &mut Vec<String>) {
    for (i, item) in vec.iter().enumerate() {
        match item.as_str() {
            "." | "," | ";" | ":" => vec.remove(i),
            _ => continue,
        };
    }
}*/
