use std::fmt::Formatter;

use serde::de::{Error, Visitor};

use super::*;

struct DataTypeVisitor;

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(DataTypeVisitor)
    }
}

impl<'de> Visitor<'de> for DataTypeVisitor {
    type Value = DataType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if eq_insensitive(v, &["f32", "fp32", "float32"]) {
            Ok(DataType::Float32)
        } else if eq_insensitive(v, &["u8", "uint8", "unsigned8"]) {
            Ok(DataType::Unsigned8)
        } else {
            Err(Error::custom(format!("unknown data type: {}", v)))
        }
    }
}

#[inline(always)]
fn eq_insensitive(input: &str, set: &[&str]) -> bool {
    for s in set {
        if s.eq_ignore_ascii_case(input) {
            return true;
        }
    }
    false
}