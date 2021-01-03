pub enum BlockType {
    Presentation, LectureNotes, Both
}

pub struct Block<'a> {
    pub content: &'a mut String,
    pub block_type: BlockType,
}
