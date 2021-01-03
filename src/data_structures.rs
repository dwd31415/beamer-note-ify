use strum_macros::EnumString;

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum BlockType {
    #[strum(serialize = "presentation", serialize = "p", serialize = "slides")]
    Presentation,
    #[strum(serialize = "notes", serialize = "lecture-notes", serialize = "paper")]
    LectureNotes,
    Both
}

#[derive(Debug, Clone)]
pub struct Block {
    pub content: String,
    pub block_type: BlockType,
}
