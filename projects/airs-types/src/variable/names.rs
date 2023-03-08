use std::ops::Div;
use super::*;

impl<'s, T> Add<T> for &VariableName<'s> where T: ToString {
    type Output = VariableName<'s>;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_string();
        self.store.add_name(&self.path, &rhs);
        self.sub_path(rhs.to_string())
    }
}

impl<'a, T> Div<T> for &mut VariableName<'a>
    where
        T: std::string::ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        self.sub_path(rhs.to_string())
    }
}

impl<'a, T> Div<T> for &VariableName<'a>
    where
        T: std::string::ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        self.sub_path(rhs.to_string())
    }
}

impl<'a, T> Div<T> for VariableName<'a>
    where
        T: std::string::ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        self.sub_path(rhs.to_string())
    }
}


impl<'s> VariableName<'s> {
    pub fn sub_path(&self, s: String) -> VariableName<'s> {
        let s = s.to_string();
        if s.chars().any(|x| x == '.') {
            panic!("sub name cannot contain {} {}", '.', s);
        }
        let mut path = self.path.clone();
        path.push(s);
        VariableName { path, group: self.group, store: self.store }
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


    pub fn find_name(&self, names: &[String], new: &str) {
        let mut name = new_name(names, new);
        self.variables.insert(name, Tensor::default());
    }
    pub fn add_name(&self, names: &[String], new: &str) {
        let mut name = new_name(names, new);
        if self.variables.contains_key(&name) {
            panic!("variable name {} already exists", name);
        }
        self.variables.insert(name, Tensor::default());
    }
}

fn new_name(names: &[String], new: &str) -> String {
    let mut name = names.join(VARIABLE_NAME_SEPARATOR);
    if !names.is_empty() {
        name.push('.');
    }
    name.push_str(new);
    name
}

