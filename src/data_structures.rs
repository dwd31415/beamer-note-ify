pub enum BlockType {
    Presentation, LectureNotes, Both
}

pub struct Block {
    pub content: String,
    pub block_type: BlockType,
}
