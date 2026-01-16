use crate::rdbn::binary_reader::BinaryReader;

pub struct RdbnRootEntry {
    pub(crate) type_index: i16,
    unk1: i16,
    pub(crate) value_offset: i32,
    pub(crate) value_size: i32,
    pub(crate) value_count: i32,
    pub(crate) name_hash: u32,
}

impl RdbnRootEntry {
    pub fn new(binary_reader: &mut BinaryReader) -> RdbnRootEntry {
        RdbnRootEntry { 
            type_index: binary_reader.read_i16(),
            unk1: binary_reader.read_i16(),
            value_offset: binary_reader.read_i32(),
            value_size: binary_reader.read_i32(),
            value_count: binary_reader.read_i32(),
            name_hash: binary_reader.read_u32(),
        }
    }
}