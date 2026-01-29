use crate::common::binary_reader::BinaryReader;

pub struct T2bChecksumSection {
    pub(crate) checksum_entries: Vec<T2bChecksumEntry>,
    pub(crate) string_offset: u64,
    pub(crate) string_size: i32,
}

impl T2bChecksumSection {
    pub fn read(binary_reader: &mut BinaryReader) -> Option<T2bChecksumSection> {
        let section_position = binary_reader.position();

        let checksum_header = T2bChecksumHeader::read(binary_reader);

        let string_offset = section_position as u64 + checksum_header.string_offset as u64;

        let checksum_entries = if checksum_header.count > 0 {
            read_checksum_entries(binary_reader, checksum_header.count)
        } else { return None };

        Some(T2bChecksumSection {
            checksum_entries,
            string_offset,
            string_size: checksum_header.string_size as i32,
        })
    }
}

pub struct T2bChecksumHeader {
    _size: u32,
    count: u32,
    string_offset: u32,
    string_size: u32,
}

impl T2bChecksumHeader {
    fn read(binary_reader: &mut BinaryReader) -> T2bChecksumHeader {
        T2bChecksumHeader {
            _size: binary_reader.read_u32(),
            count: binary_reader.read_u32(),
            string_offset: binary_reader.read_u32(),
            string_size: binary_reader.read_u32(),
        }
    }
}

pub struct T2bChecksumEntry {
    pub(crate) crc: u32,
    pub(crate) string_offset: u32,
}

fn read_checksum_entries(binary_reader: &mut BinaryReader, count: u32) -> Vec<T2bChecksumEntry> {
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        result.push(
            T2bChecksumEntry {
                crc: binary_reader.read_u32(),
                string_offset: binary_reader.read_u32(),
            }
        );
    };
    result
}