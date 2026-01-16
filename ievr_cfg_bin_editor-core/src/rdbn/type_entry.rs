use super::BinaryReader;

pub struct RdbnTypeEntry {
    pub(crate) name_hash: u32,
    pub(crate) unk1: u32,
    pub(crate) field_index: i16,
    pub(crate) field_count: i16,
}

impl RdbnTypeEntry {
    pub fn new(binary_reader: &mut BinaryReader) -> RdbnTypeEntry {
        RdbnTypeEntry {
            name_hash: binary_reader.read_u32(),
            unk1: binary_reader.read_u32(),
            field_index: binary_reader.read_i16(),
            field_count: binary_reader.read_i16(),
        }
    }
}
