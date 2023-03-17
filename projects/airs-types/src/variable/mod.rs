use std::ops::Add;

use dashmap::DashMap;

use crate::{Device, Tensor};

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

/// A VarStore is used to store variables used by one or multiple layers.
/// It specifies a single device where all variables are stored.
#[derive(Debug)]
pub struct VariableStore {
    variables: DashMap<String, Tensor>,
    device: Device,
}

/// A variable store with an associated path for variables naming.
#[derive(Clone)]
pub struct VariableName<'s> {
    path: Vec<String>,
    index: u8,
    store: &'s VariableStore,
}

impl VariableStore {
    pub fn new(device: Device) -> Self {
        Self { variables: DashMap::new(), device }
    }
}
