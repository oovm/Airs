pub use errors::{AirsErrorKind, AirsResult};

pub use crate::device::Device;

mod errors;
mod data_type;
mod variable;
mod device;

pub trait Module {}
