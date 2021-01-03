use std::primitive::str;
use std::str::FromStr;
use super::data_structures; 

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn parse_input(content:String) -> Option<Vec<data_structures::Block>>{
    let mut line_nr = 1;
    let mut blocks: Vec<data_structures::Block> = vec![]; 
    let mut current_block = data_structures::Block{content: "".to_string().to_owned(), block_type : data_structures::BlockType::Both};
    let mut inside_block = false;
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
                match tokens[0] {
                    "begin-block" => {
                        if tokens.len() == 2{
                            if inside_block { println!("Error: Block started in line {} lies in other block. Nested blocks are not allowed.", line_nr); return None; }
                            let block_type = data_structures::BlockType::from_str(tokens[1]).expect(&format!("Error: Invalid block type in line {}.", line_nr));
                            inside_block = true;
                            // append the current block
                            blocks.push(current_block.clone());
                            println!("{}",current_block.content.as_str());
                            // start a new block
                            current_block = data_structures::Block{content: "".to_string().to_owned(), block_type: block_type}; 
                        } else {
                            println!("Error: Syntax error in line {}. Should be \"@bni begin-block type\" followed by a new line.", line_nr);
                            return None;
                        }
                    }
                    "end-block" => {
                        blocks.push(current_block);
                        current_block = data_structures::Block{content: "".to_string().to_owned(), block_type : data_structures::BlockType::Both};
                        if !inside_block { println!("Error: Block ended in line {} that never started.", line_nr); return None; }
                        inside_block = false;
                    }
                    _ => {
                        println!("Error: Invalid command following @bni in line {}", line_nr);
                    }
                }
            }
            else{
                println!("Warning: In line {} @bni is invoked without a follow-up command.", line_nr);
            }
        } else {
            current_block.content.push_str(full_line);
        }
        if line_without_whitespace=="%@bni" {
            println!("Warning: In line {} @bni is invoked without a follow-up command.", line_nr);
        }
        line_nr = line_nr + 1;
    }
    return Some(blocks);
}
