
#[derive(Debug)]
pub struct ResultState<'a> {
    names:Vec<WordUsage>,
    name_set:&'a Vec<String>
}

#[derive(Debug, Clone)]
struct WordUsage {
    name_index:usize,
    letter:char
}

struct SearchState {
    word_progress:usize,
    names:Vec<WordUsage>
}
impl SearchState{
    pub fn new() ->Self{
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


pub fn display_result(res:&ResultState){
    let mut display_lines:Vec<(usize, String)> = vec![];
    let mut max_left = 0;

    for usage in res.names.iter(){
        let name = res.name_set.get(usage.name_index).unwrap();
        let letter_index = name.find(usage.letter).unwrap();
        if letter_index>max_left {max_left=letter_index}
        display_lines.push((letter_index, name.to_string()));
    }

    println!("{} of {} names used: ", res.names.len(), res.name_set.len());

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


pub struct Generator<'a>{
    pub(crate) name_set:&'a Vec<String>,
}

impl<'a> Generator<'_> {
    pub fn new(name_set: &'a Vec<String>) -> Generator {
        Generator {name_set }
    }

    pub fn generate_one(&self, word:&String) -> Option<ResultState> {
        self.try_next(SearchState::new(), &word)
    }

    //TODO: Implement function to return all possible combinations
    pub fn generate_all(&self, word:&String){
        
    }

    fn try_next(&self, current_state: SearchState, word: &str) -> Option<ResultState> {
        if current_state.word_progress>=word.len() {
            return Some(ResultState{
                names: current_state.names.to_vec(),
                name_set: self.name_set,
            })
        }
        let next_letter = word.chars().nth(current_state.word_progress).unwrap();
        let possible_names:Vec<_> = self.name_set.iter().enumerate().filter(
            |(index, name)|{name.contains(next_letter)&&!current_state.uses_name(index)}
        ).collect();
        for (name_index, _name) in possible_names.iter() {
            let mut next_names = current_state.names.to_vec();
            next_names.push(WordUsage{
                name_index: *name_index,
                letter: next_letter,
            });
            let next_state = SearchState {
                word_progress: current_state.word_progress+1,
                names: next_names,
            };
            let result = self.try_next(next_state, word);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

