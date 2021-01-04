use std::ops::Range;

static BEAMER_ENV: [&str; 1] = ["frame"];

pub fn find_closure(string: &String, open: char, close: char, start: usize) -> Option<usize>{
    let relevant_string = string.split_at(start).1;
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
    let remove_ctrl_commands = [("item",false,false), ("uncover",true,true)];
    for (command, delete_command, delete_braces) in &remove_ctrl_commands {
        let pattern = format!("\\{}", command);
        let mut finished = false;
        let mut last_starting_position = 0;
        while !finished {
            let starting_pos_maybe = code[last_starting_position..].find(&pattern);
            let mut deletions_counter = 0;
            match starting_pos_maybe{
                Some(starting_pos_init) => {
                    let mut starting_pos = starting_pos_init + last_starting_position;
                    // are we already in an infinite loop? (If we don't do something about it.)
                    if starting_pos <= last_starting_position {
                        finished = true;
                    }
                    if code.len() < starting_pos + pattern.len() {
                        // @TODO: Better error message
                        println!("Fatal LaTeX error.");
                        return None;
                    }
                    let mut extension = 0;
                    // this has to be done before the starting position is modified!
                    if code.as_str()[starting_pos + pattern.len()..].chars().next().unwrap() == '<' {
                        extension += find_closure(&code,'<','>',starting_pos + pattern.len()).expect("Fatal LaTeX error: < not closed.");
                    }
                    if !delete_command {
                        starting_pos += pattern.len();
                    } else {
                        extension += pattern.len();
                    }
                    code.replace_range(starting_pos .. starting_pos + extension, "");
                    deletions_counter += extension;
                    if *delete_braces {
                        // now the command has been deleted to the relevant starting character is the
                        // brace (is this the correct singular form?) if there is one
                        if code.as_str()[starting_pos..].chars().next().unwrap() == '{' {
                            // explains the -2: the deletion will only be applied after the initial brace is
                            // deleted.
                            let end_brace = starting_pos + find_closure(&code,'{','}',starting_pos).expect("Fatal LaTeX error: { not closed.") - 2;
                            code.replace_range(starting_pos..starting_pos+1,"");
                            code.replace_range(end_brace..end_brace+1,"");
                            deletions_counter += 2;
                        }
                    }
                    last_starting_position = starting_pos;
                },
                None => {finished=true;}
            }
            last_starting_position -= deletions_counter;
        }
    }
    return Some(code);
}
