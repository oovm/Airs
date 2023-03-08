use std::ops::Div;

use super::*;

impl<'s, T> Add<T> for &VariableName<'s> where T: ToString {
    type Output = VariableName<'s>;

    fn add(self, rhs: T) -> Self::Output {
        let path = self.store.add_name(&self.path, rhs);
        VariableName {
            path,
            store: &self.store,
        }
    }
}

impl<'a, T> Div<T> for &mut VariableName<'a>
    where
        T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.add_name(&self.path, rhs);
        VariableName {
            path,
            store: &self.store,
        }
    }
}

impl<'a, T> Div<T> for &VariableName<'a>
    where
        T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.add_name(&self.path, rhs);
        VariableName {
            path,
            store: &self.store,
        }
    }
}

impl<'a, T> Div<T> for VariableName<'a>
    where
        T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.add_name(&self.path, rhs);
        VariableName {
            path,
            store: &self.store,
        }
    }
}


impl<'s> VariableName<'s> {
    pub fn sub_path(&self, s: String) -> VariableName<'s> {
        let s = s.to_string();
        if s.chars().any(|x| x == '.') {
            panic!("sub name cannot contain {} {}", '.', s);
        }
        todo!()
        // let mut path = self.path.clone();
        // path.push(s);
        // VariableName { path, store: &self.store }
    }
}


impl VariableStore {
    pub fn root(&self) -> VariableName {
        VariableName {
            path: vec![],
            store: self,
        }
    }
    pub fn find_name<T: ToString>(&self, names: &[String], new: T) -> Vec<String> {
        let (name, names) = new_name(names.to_vec(), new.to_string());
        if self.variables.contains_key(&name) {
            todo!()
        }
        self.variables.insert(name, Tensor::default());
        names
    }
    pub fn add_name<T: ToString>(&self, names: &[String], new: T) -> Vec<String> {
        let (name, names) = new_name(names.to_vec(), new.to_string());
        if self.variables.contains_key(&name) {
            todo!()
        }
        self.variables.insert(name, Tensor::default());
        names
    }
}

#[inline(always)]
fn new_name(mut names: Vec<String>, mut name: String) -> (String, Vec<String>) {
    names.push(name);
    name = names.join(VARIABLE_NAME_SEPARATOR);
    (name, names)
}

