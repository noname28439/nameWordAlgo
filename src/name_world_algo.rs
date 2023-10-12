
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
        let letter_index = name.chars().position(|c| c == usage.letter).unwrap();
        if letter_index>max_left {max_left=letter_index}
        display_lines.push((letter_index, name.to_string()));
    }

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
    pub fn new(name_set: &'a Vec<String>) -> Generator {Generator {name_set}}

    pub fn generate(&self, word:&str, max:usize) -> Vec<ResultState>{
        let mut results:Vec<ResultState> = vec![];
        self.try_next(SearchState::new(), &word, &mut results, max);
        results
    }

    pub fn generate_fix_first(&self, word:&str, max:usize, list_index:usize) -> Vec<ResultState>{
        let first_letter = word.chars().nth(0).unwrap();
        assert!(self.name_set.get(list_index).unwrap().contains(word.chars().nth(0).unwrap()),
                "Name at index {} doesn't contain the letter \"{}\"", list_index, first_letter);

        let mut results:Vec<ResultState> = vec![];
        self.try_next(SearchState{
            word_progress: 1,
            names: vec![WordUsage{
                letter: first_letter,
                name_index: list_index
            }]
        }, &word, &mut results, max);
        results
    }

    fn try_next(&'a self, current_state: SearchState, word: &str, results: &mut Vec<ResultState<'a>>, max:usize) {
        if max!=usize::MAX && results.len()>=max {return}
        if current_state.word_progress>=word.chars().count() {
            results.push(ResultState{
                names: current_state.names.to_vec(),
                name_set: self.name_set,
            });
            return;
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
            self.try_next(next_state, word, results, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_names() -> Vec<String>{
        vec!["joe ratata".to_string(), "jake lakelbake".to_string(), "sam smankalanka".to_string(), "mae traco".to_string()]
    }

    #[test]
    fn test_algo_fail() {
        let default_names = default_names();
        let gen = Generator::new(&default_names);
        let res = gen.generate("xxxx", usize::MAX);

        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_algo_result_quantity() {
        let default_names = default_names();
        let gen = Generator::new(&default_names);

        assert_eq!(gen.generate("test", usize::MAX).len(), 2);
        assert_eq!(gen.generate("lol", usize::MAX).len(), 4);

    }

    #[test]
    fn test_algo_result_quality() {
        let default_names = default_names();
        let gen = Generator::new(&default_names);
        let res = gen.generate("test", usize::MAX);
        let example_res = res.get(0).unwrap();

        for usage in example_res.names.iter(){
            let name = example_res.name_set.get(usage.name_index).unwrap();

            assert!(name.contains(usage.letter));
        }
    }

    #[test]
    fn test_algo_first_fix() {
        let default_names = default_names();
        let gen = Generator::new(&default_names);
        let res = gen.generate_fix_first("test", usize::MAX, 0);

        //make sure the first used name is always the specified one
        for cres in &res{
            assert_eq!(cres.names.get(0).unwrap().name_index, 0)
        }
        assert_eq!(res.len(), 1)
    }

    #[test]
    #[should_panic]
    fn test_algo_first_fix_invalidity() {
        let default_names = default_names();
        let gen = Generator::new(&default_names);

        //invalid as "sam smankalanka" doesn't contain the first letter 't'
        let res = gen.generate_fix_first("test", usize::MAX, 2);
    }
}
