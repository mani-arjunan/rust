use std::env;
use std::fs;

#[derive(Debug)]
struct Config<'a> {
    query: Option<&'a String>,
    file_path: Option<&'a String>,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Config<'a> {
        Config {
            query: args.get(2),
            file_path: args.get(3),
        }
    }
}

fn search(search_str: &String, contents: &String) -> String {
    let contents_arr: Vec<char> = (*contents).chars().collect();
    let search_str_arr: Vec<char> = search_str.chars().collect();

    // i know this is bad
    for i in 0..contents_arr.len() {
        let mut j = 0;

        while j < search_str_arr.len() && contents_arr[i + j] == search_str_arr[j] {
            j += 1;
        }

        // println!("{}", j);
        if j == search_str_arr.len() {
            return String::from(search_str);
        }
    }

    return String::from("Not Found");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let mut query: &String = &String::from("");
    let mut file_path: &String = &String::from("");

    if let Some(q) = config.query {
        query = &q;
    }
    if let Some(fp) = config.file_path {
        file_path = &fp;
    }

    let full_path = format!("./src/{}.txt", file_path);
    let contents = fs::read_to_string(full_path);

    if let Ok(content) = contents {
        println!("{}", search(query, &content));
    }
}
