var names = [
    "joe ratatata",
    "jake lakelbake",
    "sam smankalanka",
    "mae traco"
]

function generate(WORD){
    function try_next(state){
        //console.info("### Try - ",state);
        if(state.names.length>=WORD.length) return state;
        let next_letter = WORD[state.letter];
        let possible_names = names.filter((name)=>(name.includes(next_letter) && !state.names.includes(name)))
        //console.info(`pos next(${next_letter}): `,possible_names);
        for(let next_name of possible_names){
            //console.info("## Next Name - ",next_name);
            let next_state = {letter: state.letter+1, names: [...state.names, next_name]}
            //console.info("Next State: ", next_state);
            let res = try_next(next_state);
            if(res!=undefined) return res; 
        }
    }
    return try_next({letter: 0, names: []}).names
}

console.log(`Result: `, generate("test"));
