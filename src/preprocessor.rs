use std::ops::Range;

static BEAMER_ENV: [&str; 1] = ["frame"];

pub fn find_closure(string: &String, open: char, close: char, start: usize) -> Option<usize>{
    let relevant_string = string.split_at(start).1;
    println!("{}",relevant_string);
    let mut counter = 0;
        let mut offset: usize = 0;
        for chr in relevant_string.chars() {
            if chr == open { counter += 1; }
            if chr == close { counter -= 1; }
            offset += chr.len_utf8();
            if counter == 0 { return Some(offset); }
        }
        return None;
    }

pub fn debeamerize(rawcode: String) -> Option<String> {
    let mut code = rawcode;
    let mut marked_for_deletion: Vec<Range<usize>> = Vec::new();
    for env in BEAMER_ENV.iter() {
        let begin = format!("\\begin{{{}}}", env);
        let end = format!("\\end{{{}}}", env);
        let mut finished = false;
        while !finished {
            let starting_pos_maybe = code.find(&begin);
            match starting_pos_maybe{
                Some(starting_pos) => {
                    if code.len() < starting_pos + begin.len() {
                        // @TODO: Better error message
                        println!("Fatal LaTeX error.");
                        return None;
                    }
                    marked_for_deletion.push(starting_pos..starting_pos+begin.len());
                    let mut extension = 0;
                    if code.as_str()[starting_pos + begin.len()..].chars().next().unwrap() == '[' {
                        extension += find_closure(&code,'[',']',starting_pos + begin.len()).expect("Fatal LaTeX error: [ no closed.");
                    }
                    if code.as_str()[starting_pos + begin.len()..].chars().next().unwrap() == '{' {
                        extension += find_closure(&code,'{','}',starting_pos + begin.len()).expect("Fatal LaTeX error: { no closed.");
                    }
                    code.replace_range(starting_pos .. starting_pos + begin.len() + extension, "");
                },
                None => {finished=true;}
            }
        }
        // quickly remove the \end{...} tags
        finished = false;
        while !finished {
            let starting_pos_maybe = code.find(&end);
            match starting_pos_maybe{
                Some(starting_pos) => {
                    if code.len() < starting_pos + 1 {
                        // @TODO: Better error message
                        println!("Fatal LaTeX error.");
                        return None;
                    }
                    code.replace_range(starting_pos .. starting_pos + end.len(), "");
                },
                None => {finished=true;}
            }
        }
    }
    return Some(code);
}
