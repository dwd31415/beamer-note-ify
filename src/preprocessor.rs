static beamer_env: [&str; 1] = ["frame"];

pub fn find_closure(string: &String, open: char, close: char, start: usize) -> Option<usize>{
    let mut end = start;
    let relevant_string = string.split_at(start).1;
    println!("{}",relevant_string);
    let mut counter = 0;
    let mut offset: usize = 0;
    for chr in relevant_string.chars() {
        if chr == open { counter += 1; }
        if chr == close { counter -= 1; }
        offset += 1;
        if counter == 0 { return Some(offset); }
    }
    return None;
}

pub fn debeamerize(code: String) -> Option<String> {
    for env in beamer_env.iter() {
        let begin = format!("\\begin{{{}}}", env);
        let end = format!("\\end{{{}}}", env);
        println!("{}",begin);
        let mut exmp = String::from("[hello world[][][]] hi]{}{}fvreverv").to_owned();
        let exmp_end = find_closure(&exmp,'[',']',0).unwrap();
        exmp.replace_range((0..exmp_end),"");
        println!("{}", exmp);
        for (starting_pos,_) in code.match_indices(&begin) {
            if code.len() < starting_pos + 1 {
                // @TODO: Better error message
                println!("Fatal LaTeX error.");
                return None;
            }
            if &code.as_str()[starting_pos..starting_pos+1] == "[" {

            }
        }
    }
    return Some(code);
}
