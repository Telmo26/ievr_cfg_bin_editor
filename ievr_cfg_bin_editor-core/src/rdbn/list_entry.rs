#[derive(Debug)]
pub struct RdbnListEntry {
    pub name: String,
    pub type_index: usize,
    pub values: Vec<Vec<Vec<RdbnValue>>>,
}

#[derive(Debug)]
pub enum RdbnValue {
    Bool(bool),
    Byte(u8),
    Short(i16),
    Int(i32),
    Uint(u32),
    Float(f32),
    Hash(u32),
    String(String),
    Bytes(Vec<u8>),
    Float4([f32; 4]),
    Short2([i16; 2]),
}