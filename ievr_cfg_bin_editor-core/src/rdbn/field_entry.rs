use crate::rdbn::binary_reader::BinaryReader;

pub struct RdbnFieldEntry {
    pub(crate) name_hash: u32,
    pub(crate) r#type: i16,
    pub(crate) type_category: i16,
    pub(crate) value_size: i32,
    pub(crate) value_offset: i32,
    pub(crate) value_count: i32,
}

impl RdbnFieldEntry {
    pub fn new(binary_reader: &mut BinaryReader) -> RdbnFieldEntry {
        RdbnFieldEntry {
            name_hash: binary_reader.read_u32(),
            r#type: binary_reader.read_i16(),
            type_category: binary_reader.read_i16(),
            value_size: binary_reader.read_i32(),
            value_offset: binary_reader.read_i32(),
            value_count: binary_reader.read_i32(),
        }
    }
}
