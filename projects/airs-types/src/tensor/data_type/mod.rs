use serde::{Deserialize, Deserializer, Serialize};

mod der;


#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum DataType {
    Unsigned8,
    Float32,
    BrainFloat16,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Float32
    }
}

