use std::collections::HashMap;

use crate::common::{binary_reader::BinaryReader, compute_crc32_jam, compute_crc32_standard};

use super::{
    T2b,
    HashType,
    T2bEntryValue, T2bEntry,
    T2bValueType, T2bValue,
    ValueLength,
    footer::T2bFooter,
    entry_section::T2bEntrySection,
    checksum_section::{T2bChecksumSection, T2bChecksumEntry},
};

const MINIMUM_SIZE: i32 = 0x30;

impl T2b {
    pub fn read(file: &[u8]) -> Option<T2b> {
        let mut binary_reader = BinaryReader::new(file);

        if binary_reader.file_size() < MINIMUM_SIZE as usize {
            return None;
        }

        binary_reader.set_position(binary_reader.file_size() - 0x10);

        let footer = T2bFooter::read(&mut binary_reader);
        if footer.magic != 0x62327401 { // .t2b in little-endian
            return None;
        }

        binary_reader.set_position(0);

        let entry_section = match T2bEntrySection::read(&mut binary_reader) {
            Some(data) => data,
            None => return None
        };

        // Read value string
        let value_string_data = if entry_section.string_size > 0 {
            binary_reader.set_position(entry_section.string_offset as usize);
            binary_reader.read_bytes(entry_section.string_size as usize).to_vec()
        } else {
            Vec::new()
        };

        binary_reader.seek_alignment(0x10);

        let encoding = footer.encoding;
        let checksum_position = binary_reader.position();

        let checksum_section = T2bChecksumSection::read(&mut binary_reader)?;

        binary_reader.set_position(checksum_position + checksum_section.string_offset as usize);

        let checksum_string_data = if checksum_section.string_size > 0 {
            binary_reader.set_position(checksum_section.string_offset as usize);
            binary_reader.read_bytes(checksum_section.string_size as usize)
        } else { &[] };

        let mut hash_type = HashType::Crc32Standard;

        if checksum_section.checksum_entries.len() > 0 {
            if let Some(hash) = try_detect_hash_type(&checksum_section.checksum_entries[0], checksum_string_data, encoding) {
                hash_type = hash;
            } else {
                return None
            }
        }

        // println!("{:?}", entry_section);

        Some(T2b::create_configuration(entry_section, checksum_section, &value_string_data, checksum_string_data, encoding, hash_type))
    }

    fn create_configuration(entry_section: T2bEntrySection, checksum_section: T2bChecksumSection, value_string_data: &[u8], checksum_string_data: &[u8], encoding: i16, hash_type: HashType) -> T2b {
        let checksum_offset_lookup: HashMap<u32, u32> = checksum_section.checksum_entries.iter().map(|section| {
            (section.crc, section.string_offset - checksum_section.checksum_entries[0].string_offset)
        })
        .collect();

        let mut config_entries = Vec::with_capacity(entry_section.entries.len());

        for i in 0..entry_section.entries.len() {
            let mut config_entry_value = Vec::with_capacity(entry_section.entries[i].entry_count as usize);

            for j in 0..entry_section.entries[i].entry_count as usize {
                let entry_type = entry_section.entries[i].entry_types[j];
                let entry_value = entry_section.entries[i].entry_values[j];

                let value = match entry_type {
                    T2bValueType::String => {
                        if entry_value < 0 {
                            T2bValue::String(String::new())
                        } else {
                            T2bValue::String(read_string(value_string_data, entry_value as u32, encoding).unwrap())
                        }
                    },
                    T2bValueType::Integer => match entry_section.value_length {
                        ValueLength::Int => T2bValue::Integer(entry_value as i32),
                        ValueLength::Long => T2bValue::Long(entry_value as i64),
                    },
                    T2bValueType::FloatingPoint => {
                        match entry_section.value_length {
                            ValueLength::Int => T2bValue::F32(f32::from_bits(entry_value as u32)),
                            ValueLength::Long => T2bValue::F64(f64::from_bits(entry_value as u64)),
                        }
                    },
                    T2bValueType::Invalid => panic!("Encountered an invalid value type!"),
                };
                config_entry_value.push(
                    T2bEntryValue {
                        r#type: entry_type,
                        value,
                    }
                );
            }

            let name = read_string(checksum_string_data, checksum_offset_lookup[&entry_section.entries[i].crc32], encoding).unwrap();

            config_entries.push(
                T2bEntry {
                    name,
                    values: config_entry_value,
                }
            );
        }

        T2b {
            entries: config_entries,
            encoding,
            value_length: entry_section.value_length,
            hash_type,
        }
    }
}

fn try_detect_hash_type(entry: &T2bChecksumEntry, string_data: &[u8], encoding: i16) -> Option<HashType> {
    let string_value = read_string(string_data, entry.string_offset, encoding).unwrap();

    let hash_types = [HashType::Crc32Standard, HashType::Crc32Jam];
    
    for hash_type in hash_types {
        let string_hash = match hash_type {
            HashType::Crc32Standard => compute_crc32_standard(string_value.as_bytes()),
            HashType::Crc32Jam => compute_crc32_jam(string_value.as_bytes()),
        };

        if string_hash == entry.crc {
            return Some(hash_type)
        }
    }
    None
}

fn read_string(string_data: &[u8], offset: u32, _encoding: i16) -> Option<String> {
    let mut end_offset = offset;
    while string_data[end_offset as usize] != 0 {
        end_offset += 1;
    }

    String::from_utf8(string_data[offset as usize..end_offset as usize].to_vec()).ok()
}