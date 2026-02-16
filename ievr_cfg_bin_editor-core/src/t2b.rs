use serde::{Deserialize, Serialize};

pub use crate::{
    t2b::entry_section::{T2bValueType, ValueLength},
};

mod footer;
mod entry_section;
mod checksum_section;

mod reader;
mod writer;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct T2bEntry {
    pub name: String,
    pub values: Vec<T2bEntryValue>,
}

#[derive(Debug, Clone)]
pub struct T2bEntryValue {
    pub r#type: T2bValueType,
    pub value: T2bValue,
}

#[derive(Debug, Clone)]
pub enum T2bValue {
    String(String),
    Integer(i32),
    Long(i64),
    F32(f32),
    F64(f64),
}