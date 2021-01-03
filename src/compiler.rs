use super::data_structures;
use super::BuildMode;

pub fn compile(blocks:Vec<data_structures::Block>, build_mode: BuildMode) -> Option<String> {
    let mut teX_code = "".to_string().to_owned();
    for block in blocks {
        println!("{:?}",data_structures::check_block_type_compatility(build_mode, block.block_type));
    }
    return Some(teX_code);
}
