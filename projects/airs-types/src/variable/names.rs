use crate::Tensor;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Div,
};

use super::*;

impl<'s> Debug for VariableName<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VariableName").field(&self.to_string()).field(&self.store.device).finish()
    }
}

impl<'s> Display for VariableName<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            return Ok(());
        }
        f.write_str(&self.pure_name())?;
        if self.index != 0 {
            f.write_char('_')?;
            f.write_str(&self.index.to_string())?
        }
        Ok(())
    }
}

impl<'s, T> Add<T> for &mut VariableName<'s>
where
    T: ToString,
{
    type Output = VariableName<'s>;

    fn add(self, rhs: T) -> Self::Output {
        self.store.add_name(self, rhs)
    }
}

impl<'s, T> Add<T> for &VariableName<'s>
where
    T: ToString,
{
    type Output = VariableName<'s>;

    fn add(self, rhs: T) -> Self::Output {
        self.store.add_name(self, rhs)
    }
}

impl<'s, T> Add<T> for VariableName<'s>
where
    T: ToString,
{
    type Output = VariableName<'s>;

    fn add(self, rhs: T) -> Self::Output {
        self.store.add_name(&self, rhs)
    }
}

impl<'a, T> Div<T> for &mut VariableName<'a>
where
    T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.div_name(&self.path, rhs);
        VariableName { path, index: 0, store: &self.store }
    }
}

impl<'a, T> Div<T> for &VariableName<'a>
where
    T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.div_name(&self.path, rhs);
        VariableName { path, index: 0, store: &self.store }
    }
}

impl<'a, T> Div<T> for VariableName<'a>
where
    T: ToString,
{
    type Output = VariableName<'a>;

    fn div(self, rhs: T) -> Self::Output {
        let path = self.store.div_name(&self.path, rhs);
        VariableName { path, index: 0, store: &self.store }
    }
}

impl<'s> VariableName<'s> {
    fn insert_deduplication(&mut self) {
        self.index += 1;
        let name = self.to_string();
        if self.store.variables.contains_key(&name) {
            if self.index == u8::MAX {
                panic!("Too many variables with the same name `{}`", self.pure_name());
            }
            self.insert_deduplication()
        }
        else {
            self.store.variables.insert(name, Tensor::default());
        }
    }
    #[inline(always)]
    fn pure_name(&self) -> String {
        self.path.join(VARIABLE_NAME_SEPARATOR)
    }
}

impl VariableStore {
    pub fn root(&self) -> VariableName {
        VariableName { path: vec![], index: 0, store: self }
    }
    /// Create a new variable name if not exist, or reuse existence variable.
    pub fn div_name<T: ToString>(&self, names: &[String], new: T) -> Vec<String> {
        let (name, names) = new_name(names.to_vec(), new.to_string());
        if self.variables.contains_key(&name) {
            return names;
        }
        self.variables.insert(name, Tensor::default());
        names
    }
    /// Create a new variable in any case.
    pub fn add_name<T: ToString>(&self, old: &VariableName, new: T) -> VariableName {
        assert!(std::ptr::eq(old.store, self), "VariableName must be created by this VariableStore");
        let mut path = old.path.to_vec();
        path.push(new.to_string());
        let mut insert = VariableName { path, index: 0, store: &self };
        if self.variables.contains_key(&insert.to_string()) {
            insert.insert_deduplication();
        }
        self.variables.insert(insert.to_string(), Tensor::default());
        insert
    }
}

#[inline(always)]
fn new_name(mut names: Vec<String>, mut name: String) -> (String, Vec<String>) {
    names.push(name);
    name = names.join(VARIABLE_NAME_SEPARATOR);
    (name, names)
}
