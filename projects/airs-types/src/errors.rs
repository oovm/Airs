#[derive(Debug, Copy, Clone)]
pub enum AirsErrorKind {
    UnknownError
}

pub struct AirsError {
    pub kind: Box<AirsErrorKind>,
}


pub type Result<T> = std::result::Result<T, AirsErrorKind>;
