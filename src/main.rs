mod name_world_algo;

use std::fs::read_to_string;
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
        Err(_e) =>{
            println!("File './names.txt' could not be read...");
            std::process::exit(0);
        }
    };
    let lines = content.lines().map(|line|{line.to_lowercase()}).collect();

    let builder = name_world_algo::Generator::new(&lines);

    let generated = builder.generate(&word, usize::MAX);

    println!("Generated {} combinations. \nPress enter to print one at a time. \nEnter \"quit\" to terminate", generated.len());

    for res in generated{
        let mut read = String::new();
        let _ = std::io::stdin().read_line(&mut read);
        if read.starts_with("q"){
            std::process::exit(0);
        }
        display_result(&res);
    }

}
