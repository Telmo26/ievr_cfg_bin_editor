use super::BinaryReader;

#[derive(Debug)]
pub struct RdbnHeader {
    pub(crate) magic: u32,
    pub(crate) _header_size: i16,
    pub(crate) _version: i32,
    pub(crate) data_offset: i16,
    pub(crate) _data_size: i32,

    pub(crate) type_offset: i16,
    pub(crate) type_count: i16,
    pub(crate) field_offset: i16,
    pub(crate) field_count: i16,
    pub(crate) root_offset: i16,
    pub(crate) root_count: i16,
    pub(crate) string_hash_offset: i16,
    pub(crate) string_offsets_offset: i16,
    pub(crate) hash_count: i16,
    pub(crate) value_offset: i16,
    pub(crate) string_offset: i32,
}

impl RdbnHeader {
    pub fn new(binary_reader: &mut BinaryReader) -> RdbnHeader {
        let magic       = binary_reader.read_u32();
        let _header_size = binary_reader.read_i16();
        let _version     = binary_reader.read_i32();
        let data_offset = binary_reader.read_i16();
        let _data_size   = binary_reader.read_i32();

        // Skip 0x14 bytes (unknown / reserved)
        binary_reader.skip(0x14);

        let type_offset             = binary_reader.read_i16();
        let type_count              = binary_reader.read_i16();
        let field_offset            = binary_reader.read_i16();
        let field_count             = binary_reader.read_i16();
        let root_offset             = binary_reader.read_i16();
        let root_count              = binary_reader.read_i16();
        let string_hash_offset      = binary_reader.read_i16();
        let string_offsets_offset   = binary_reader.read_i16();
        let hash_count              = binary_reader.read_i16();
        let value_offset            = binary_reader.read_i16();
        let string_offset           = binary_reader.read_i32();

        RdbnHeader {
            magic,
            _header_size,
            _version,
            data_offset,
            _data_size,

            type_offset,
            type_count,
            field_offset,
            field_count,
            root_offset,
            root_count,
            string_hash_offset,
            string_offsets_offset,
            hash_count,
            value_offset,
            string_offset,
        }
    }
}