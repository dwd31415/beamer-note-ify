use super::data_structures;
use super::preprocessor;
use super::BuildMode;

pub fn compile(blocks:Vec<data_structures::Block>, build_mode: BuildMode) -> Option<String> {
    let mut teX_code = "".to_string().to_owned();
    for block in blocks {
        if data_structures::check_block_type_compatility(build_mode, block.block_type) {
            let content = match build_mode {
                BuildMode::LectureNotes => preprocessor::debeamerize(block.content),
                BuildMode::Presentation => block.content,
                BuildMode::Both => String::from("")
            };
            teX_code.push_str(content.as_str());
        }
    }
    return Some(teX_code);
}
