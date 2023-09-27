mod name_world_algo;

use std::fs::read_to_string;
use std::string::ToString;
use std::env;
use crate::name_world_algo::display_result;

static FILE_NAME:&str = r"./names.txt";


fn main() {
    let _ = enable_ansi_support::enable_ansi_support();

    let word = match env::args().nth(1) {
        Some(word) => word.to_lowercase(),
        None => String::from("test")
    };

    let content = match read_to_string(FILE_NAME) {
        Ok(c) => c,
        Err(e) =>{
            println!("File './names.txt' could not be read...");
            std::process::exit(0);
        }
    };
    let lines = content.lines().map(|line|{line.to_lowercase()}).collect();

    let builder = name_world_algo::Generator::new(&lines);

    let res = match builder.build_one(&word) {
        Some(res) => res,
        None => {
            println!("Can't form '{}' from supplied wordlist...", word);
            std::process::exit(0);
        }
    };

    display_result(&res);

}
