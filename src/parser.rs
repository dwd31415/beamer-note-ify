use std::primitive::str;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn parse_input(content:String) {
    let mut line_nr = 1;
    for full_line in content.lines() {
        // check if the line is a latex comment and start with @bni
        let line_without_whitespace = remove_whitespace(full_line);
        if line_without_whitespace.starts_with("%@bni") && !(line_without_whitespace=="%@bni"){
            // okay then we have a possible command
            // remove the irrelevant bits
            let line = full_line.split_at(5 + full_line.find("@bni").expect(&format!("Parsing error in line {}, please ensure that @bni is written correctly.",line_nr))).1;
            // remove dupicate whitespaces
            let mut line_vec: Vec<char> = line.chars().collect();
            line_vec.dedup_by(|c,d| c.is_whitespace() && d.is_whitespace());
            let line_clean = &line_vec.iter().collect::<String>();
            let tokens : Vec<&str> = line_clean.split(' ').collect();
            //check that there are tokens to process
            if tokens.len() > 0 {
                
            }
            else{
                println!("Warning: In line {} @bni is invoked without a follow-up command.", line_nr);
            }
        }
        if line_without_whitespace=="%@bni" {
            println!("Warning: In line {} @bni is invoked without a follow-up command.", line_nr);
        }
        line_nr = line_nr + 1;
    }
}
