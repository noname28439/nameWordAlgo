use std::env;
use std::fs::read_to_string;
use std::string::ToString;

#[derive(Debug, Clone)]
struct WordUsage {
    name_index:usize,
    letter:char
}

#[derive(Debug)]
struct ResultState {
    names:Vec<WordUsage>
}

struct SearchState {
    word_progress:usize,
    names:Vec<WordUsage>
}
impl SearchState{
    pub fn begin()->Self{
        SearchState{
            word_progress: 0,
            names: vec![]
        }
    }
    pub fn uses_name(&self, name_index:&usize)->bool{
        let mut found:bool = false;
        for name in &self.names{
            if &name.name_index == name_index {
                found=true;
            }
        }
        found
    }
}

unsafe fn try_next(current_state: SearchState, word:&str) -> Option<ResultState>{
    if current_state.word_progress>=word.len() {
        return Some(ResultState{
            names: current_state.names.to_vec(),
        })
    }
    let next_letter = word.chars().nth(current_state.word_progress).unwrap();
    let possible_names:Vec<_> = AVAILABLE_NAMES.iter().enumerate().filter(
        |(index, name)|{name.contains(next_letter)&&!current_state.uses_name(index)}
    ).collect();
    for (name_index, name) in possible_names.iter() {
        let mut next_names = current_state.names.to_vec();
        next_names.push(WordUsage{
            name_index: *name_index,
            letter: next_letter,
        });
        let next_state = SearchState {
            word_progress: current_state.word_progress+1,
            names: next_names,
        };
        let result = try_next(next_state, word);
        if result.is_some() {
            return result;
        }
    }
    None
}

static FILE_NAME:&str = r"./names.txt";

static mut AVAILABLE_NAMES: Vec<String> = vec![];

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

    let names:Vec<_> = content.lines().map(|line|{line.to_lowercase()}).collect();
    unsafe {
        AVAILABLE_NAMES = names;

        let mut display_lines:Vec<(usize, String)> = vec![];

        let mut max_left = 0;

        let res = match try_next(SearchState::begin(), word.as_str()) {
            Some(res) => res,
            None => {
                println!("Can't form '{}' from supplied wordlist...", word);
                std::process::exit(0);
            }
        };
        for usage in res.names.iter(){
            let name = AVAILABLE_NAMES.get(usage.name_index).unwrap();
            let letter_index = name.find(usage.letter).unwrap();
            if letter_index>max_left {max_left=letter_index}
            display_lines.push((letter_index, name.to_string()));
        }

        //Display result

        println!("{} of {} names used: ", res.names.len(), AVAILABLE_NAMES.len());

        for (letter_index, name) in display_lines{
            let offset = max_left-letter_index;
            for _ in 0..offset {
                print!(" ");
            }
            for (i, c) in name.chars().enumerate(){
                if i == letter_index {
                    print!("\x1b[31m{}\x1b[0m", c.to_string().to_uppercase());
                }else {
                    print!("{}", c);
                }
            }
            print!("\n");
        }

    }

}
