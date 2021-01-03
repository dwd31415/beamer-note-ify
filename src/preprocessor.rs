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
        //let mut exmp = String::from("[helloβ world[][][]] hi]{}{}fvreverv").to_owned();
        //let exmp_end = find_closure(&exmp,'[',']',0).unwrap();
        //exmp.replace_range((0..exmp_end),"");
        for (starting_pos,_) in code.match_indices(&begin) {
            if code.len() < starting_pos + 1 {
                // @TODO: Better error message
                println!("Fatal LaTeX error.");
                return None;
            }
            marked_for_deletion.push(starting_pos..starting_pos+begin.len());
            println!("{:?}",starting_pos..starting_pos+begin.len());
            if &code.as_str()[starting_pos..starting_pos+1] == "[" {

            }
        }
        for (starting_pos,_) in code.match_indices(&end){
            marked_for_deletion.push(starting_pos..starting_pos+end.len());
        }
    }
    let mut offset: usize = 0;
    for deletion_range in marked_for_deletion {
        code.replace_range(deletion_range.start - offset .. deletion_range.end - offset, "");
        offset += deletion_range.end - deletion_range.start;
    }
    return Some(code);
}
