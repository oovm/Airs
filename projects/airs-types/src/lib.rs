pub use crate::{
    data_type::DataType,
    device::Device,
    errors::{AirsError, AirsErrorKind, AirsResult},
    module::Module,
    variable::{VariableName, VariableStore, VARIABLE_NAME_SEPARATOR},
};

mod data_type;
mod device;
mod errors;
mod module;
mod variable;
