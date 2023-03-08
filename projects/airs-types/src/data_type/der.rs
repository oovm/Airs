use std::fmt::Formatter;

use serde::de::Visitor;

use super::*;

struct DataTypeVisitor;

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(DataTypeVisitor)
    }
}

impl<'de> Visitor<'de> for DataTypeVisitor {
    type Value = DataType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}