    use std::primitive::str;
    use indextree::Arena;

    use super::data_structures; 

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    pub fn parse_input(content:String) {
        let mut line_nr = 1;
        let arena = &mut Arena::new();
        let root_node = arena.new_node("".to_string().to_owned());
        let mut current_node = root_node; 
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
                        inside_block = true;
                        // append the current block
                        arena.get_mut(current_node).expect("Internal error.").get_mut().push_str(current_block.content.as_str());
                        println!("{}",current_block.content.as_str());
                        // start a new block
                        current_block = data_structures::Block{content: "".to_string().to_owned(), block_type : data_structures::BlockType::Both};
                        // start a new branch in the tree
                        let new_node = arena.new_node("".to_string().to_owned());
                        current_node.append(new_node, arena);
                        current_node = new_node;
                    }
                    "end-block" => {
                        let parent = current_node.ancestors(&arena).next();
                        match parent {    
                            None => {
                                println!("Error: Block that was never started ended in line {}", line_nr);
                                return ();
                            },
                            Some(parent_id) => {
                                current_node = parent_id;
                                if parent_id == root_node {
                                    inside_block=false;
                                }
                            }
                        }
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
            if inside_block {
                current_block.content.push_str(full_line);
            }
            else {
                arena.get_mut(current_node).expect("Internal error.").get_mut().push_str(full_line);
            }
        }
        if line_without_whitespace=="%@bni" {
            println!("Warning: In line {} @bni is invoked without a follow-up command.", line_nr);
        }
        line_nr = line_nr + 1;
    }
    for line in root_node.children(&arena){
        println!("{}", line);
    }
}
