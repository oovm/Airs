use std::ops::Add;

use dashmap::DashMap;

use crate::Device;

mod names;

pub const VARIABLE_NAME_SEPARATOR: &'static str = ".";

// When the variable store is frozen, the trainable_variables vector
// still contains the same tensors however these tensors are set not
// to require gradients.
#[derive(Debug)]
pub struct Variables {
    // pub named_variables: HashMap<String, Tensor>,
    // pub trainable_variables: Vec<Var>,
}

#[derive(Debug, Default)]
pub struct Tensor {}

/// A VarStore is used to store variables used by one or multiple layers.
/// It specifies a single device where all variables are stored.
#[derive(Debug)]
pub struct VariableStore {
    variables: DashMap<String, Tensor>,
    device: Device,
}

/// A variable store with an associated path for variables naming.
#[derive(Debug, Clone)]
pub struct VariableName<'s> {
    path: Vec<String>,
    store: &'s VariableStore,
}


impl VariableStore {
    pub fn new(device: Device) -> Self {
        Self {
            variables: DashMap::new(),
            device,
        }
    }
}


