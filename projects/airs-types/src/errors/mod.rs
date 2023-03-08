use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::{Path, PathBuf},
};

pub type AirsResult<T = ()> = Result<T, AirsErrorKind>;

#[derive(Debug)]
pub struct AirsError {
    pub kind: Box<AirsErrorKind>,
}

#[derive(Debug)]
pub enum AirsErrorKind {
    IoError { path: PathBuf, raw: std::io::Error },
}

impl Error for AirsError {}

impl Display for AirsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}
impl Display for AirsErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AirsErrorKind::IoError { path, raw } => {
                write!(f, "io error: {} ({:?})", raw, path)
            }
        }
    }
}

impl From<std::io::Error> for AirsErrorKind {
    fn from(raw: std::io::Error) -> Self {
        Self::IoError { path: PathBuf::new(), raw }
    }
}

impl AirsError {
    pub fn set_path<P: AsRef<Path>>(&mut self, new: P) {
        match self.kind.as_mut() {
            AirsErrorKind::IoError { path, .. } => {
                *path = new.as_ref().to_path_buf();
            }
        }
    }
    pub fn with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.set_path(path);
        self
    }
}
