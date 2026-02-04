use serde::{Serialize, Deserialize};

use crate::{rdbn::{RdbnFieldType, RdbnValue}, t2b::{T2bEntryValue, T2bValue, T2bValueType}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub(super) name: String,
    pub(super) schema: Schema,
    pub(super) rows: Vec<Row>
}

impl Table {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Row> {
        &mut self.rows
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub(super) name: String,
    pub(super) fields: Vec<Field>
}

impl Schema {
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value_type: ValueType,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub values: Vec<Vec<Value>>, // A single column can store multiple values in the RDBN data format
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueType {
    Rdbn(RdbnFieldType),
    T2b(T2bValueType),
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Byte(u8),
    Int(i32),
    Long(i64),
    Short(i16),
    UInt(u32),
    Float(f32),
    FloatLong(f64),
    String(String),

    Hash(u32),
    Bytes(Vec<u8>),
    
    Tuple2I16(i16, i16),
    Vec4F32([f32; 4]),
}

impl From<&RdbnValue> for Value {
    fn from(value: &RdbnValue) -> Self {
        match value {
            RdbnValue::Bool(v) => Value::Bool(*v),
            RdbnValue::Byte(v) => Value::Byte(*v),
            RdbnValue::Short(v) => Value::Short(*v),
            RdbnValue::Int(v) => Value::Int(*v),
            RdbnValue::Uint(v) => Value::UInt(*v),
            RdbnValue::Float(v) => Value::Float(*v),
            RdbnValue::Hash(v) => Value::Hash(*v),
            RdbnValue::String(v) => Value::String(v.clone()),
            RdbnValue::Bytes(v) => Value::Bytes(v.clone()),
            RdbnValue::Float4(v) => Value::Vec4F32(*v),
            RdbnValue::Short2(v) => Value::Tuple2I16(v[0], v[1]),
        }
    }
}

impl From<&T2bEntryValue> for Value {
    fn from(entry: &T2bEntryValue) -> Self {
        match &entry.value {
            T2bValue::String(v) => Value::String(v.clone()),
            T2bValue::Integer(v) => Value::Int(*v),
            T2bValue::Long(v) => Value::Long(*v),
            T2bValue::F32(v) => Value::Float(*v),
            T2bValue::F64(v) => Value::FloatLong(*v),
        }
    }
}