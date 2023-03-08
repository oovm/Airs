use super::*;

impl<'s, S: ToString> Add<S> for VariableName<'s> {
    type Output = Self;

    fn add(mut self, rhs: S) -> Self::Output {
        self.path.push(rhs.to_string());
        Self {
            path: self.path,
            group: self.group,
            store: &self.store,
        }
    }
}


impl VariableStore {
    pub fn root(&self) -> VariableName {
        VariableName {
            path: vec![],
            group: 0,
            store: self,
        }
    }
    pub fn find_name(&self, names: &[String]) {
        let name = names.join(VARIABLE_NAME_SEPARATOR);
        self.variables.insert(name, Tensor::default());
    }
    pub fn add_name(&self, names: &[String]) {
        let name = names.join(VARIABLE_NAME_SEPARATOR);
        self.variables.insert(name, Tensor::default());
    }
}


