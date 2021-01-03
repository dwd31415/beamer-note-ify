use strum_macros::EnumString;
use super::BuildMode;

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum BlockType {
    #[strum(serialize = "presentation", serialize = "p", serialize = "slides")]
    Presentation,
    #[strum(serialize = "notes", serialize = "lecture-notes", serialize = "paper")]
    LectureNotes,
    Both
}

pub fn check_block_type_compatility(build_mode: BuildMode, block_type: BlockType) -> bool {
    if build_mode == BuildMode::Both {return true;}
    if block_type == BlockType::Both {return true;}
    if build_mode == BuildMode::Presentation && block_type == BlockType::Presentation {return true;}
    if build_mode == BuildMode::LectureNotes && block_type == BlockType::LectureNotes {return true;}
    return false;
}

#[derive(Debug, Clone)]
pub struct Block {
    pub content: String,
    pub block_type: BlockType,
}
