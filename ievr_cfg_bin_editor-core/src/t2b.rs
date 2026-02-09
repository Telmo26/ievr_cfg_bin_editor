use serde::{Deserialize, Serialize};

use crate::Value;

pub use crate::{
    t2b::entry_section::{T2bValueType, ValueLength},
};

mod footer;
mod entry_section;
mod checksum_section;

mod reader;
mod writer;

#[derive(Debug)]
pub struct T2b {
    pub entries: Vec<T2bEntry>,
    pub encoding: i16,
    pub value_length: ValueLength,
    pub hash_type: HashType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HashType {
    Crc32Standard,
    Crc32Jam,
}

#[derive(Debug)]
pub struct T2bEntry {
    pub name: String,
    pub values: Vec<T2bEntryValue>,
}

#[derive(Debug)]
pub struct T2bEntryValue {
    pub r#type: T2bValueType,
    pub value: T2bValue,
}

#[derive(Debug)]
pub enum T2bValue {
    String(String),
    Integer(i32),
    Long(i64),
    F32(f32),
    F64(f64),
}

impl From<Value> for T2bValue {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => T2bValue::String(s),
            Value::Int(i) => T2bValue::Integer(i),
            Value::Long(l) => T2bValue::Long(l),
            Value::Float(f) => T2bValue::F32(f),
            Value::FloatLong(fl) => T2bValue::F64(fl),
            _ => panic!("Trying to insert invalid value type in T2B")
        }
    }
}