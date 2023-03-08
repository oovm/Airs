pub type AirsResult<T = ()> = Result<T, AirsErrorKind>;

pub struct AirsError {
    pub kind: Box<AirsErrorKind>,
}

#[derive(Debug, Copy, Clone)]
pub enum AirsErrorKind {
    UnknownError
}
