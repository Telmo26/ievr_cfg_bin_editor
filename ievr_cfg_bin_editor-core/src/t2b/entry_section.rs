use serde::{Deserialize, Serialize};

use crate::common::binary_reader::BinaryReader;

#[derive(Debug)]
pub struct T2bEntrySection {
    pub(crate) entries: Vec<T2bEntry>,
    pub(crate) string_offset: i64,
    pub(crate) string_size: i32,
    pub(crate) value_length: ValueLength,
}

impl T2bEntrySection {
    pub fn read(binary_reader: &mut BinaryReader) -> Option<T2bEntrySection> {
        let section_position = binary_reader.position();

        let entry_header = T2bEntryHeader::read(binary_reader);
        let string_offset = (section_position + entry_header.string_data_offset as usize) as i64;

        let mut value_length = ValueLength::Int;
        let mut entries = Vec::new();

        if entry_header.entry_count > 0 {
            match try_detect_value_length(binary_reader, entry_header.entry_count, string_offset as u64) {
                Some(length) => { 
                    entries = read_entries(binary_reader, entry_header.entry_count, length);
                    value_length = length;
                }
                None => return None,
            }
        }

        Some(T2bEntrySection {
            entries,
            string_offset,
            string_size: entry_header.string_data_length as i32,
            value_length,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueLength {
    Int = 4,
    Long = 8
}

pub struct T2bEntryHeader {
    entry_count: u32,
    string_data_offset: u32,
    string_data_length: u32,
    _string_data_count: u32,
}

impl T2bEntryHeader {
    pub fn read(binary_reader: &mut BinaryReader) -> T2bEntryHeader {
        T2bEntryHeader {
            entry_count: binary_reader.read_u32(),
            string_data_offset: binary_reader.read_u32(),
            string_data_length: binary_reader.read_u32(),
            _string_data_count: binary_reader.read_u32(),
        }
    }
}

fn try_detect_value_length(binary_reader: &mut BinaryReader, entry_count: u32, data_end_offset: u64) -> Option<ValueLength> {
    let original_pos = binary_reader.position();

    let value_lengths = [ValueLength::Int, ValueLength::Long];

    for length in value_lengths {
        if try_read_entry_section(binary_reader, entry_count, data_end_offset, length as i32) {
            binary_reader.set_position(original_pos);
            return Some(length)
        }
        binary_reader.set_position(original_pos);
    }

    return None
}

fn try_read_entry_section(binary_reader: &mut BinaryReader, entry_count: u32, data_end_offset: u64, length: i32) -> bool {
    let value_lengths = [length, length, length];

    for _ in 0..entry_count {
        if binary_reader.position() + 8 > data_end_offset as usize {
            return false;
        }

        binary_reader.skip(4);

        let count = binary_reader.read_byte() as i32;
        let types = read_entry_types(binary_reader, count);

        if binary_reader.position() as u64 > data_end_offset {
            return false
        }

        if types.iter().any(|t| *t == T2bValueType::Invalid) {
            return false
        }

        binary_reader.skip(types.iter().map(|t| value_lengths[(*t as u8) as usize] as usize).sum());
    }

    return binary_reader.position() as u64 <= data_end_offset && (data_end_offset - binary_reader.position() as u64) < 0x10;
}

fn read_entry_types(binary_reader: &mut BinaryReader, count: i32) -> Vec<T2bValueType> {
    let mut types = Vec::with_capacity(count as usize);

    for j in (0..count).step_by(4) {
        let type_chunk = binary_reader.read_byte();
        for h in 0..4 {
            if j + h >= count {
                break
            }

            types.push(T2bValueType::from((type_chunk >> h * 2) & 0x3));
        }
    }

    binary_reader.seek_alignment(4);

    return types
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum T2bValueType {
    String = 0,
    Integer = 1,
    FloatingPoint = 2,
    Invalid = 3,
}

impl From<u8> for T2bValueType {
    fn from(value: u8) -> Self {
        match value {
            0 => T2bValueType::String,
            1 => T2bValueType::Integer,
            2 => T2bValueType::FloatingPoint,
            _ => T2bValueType::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct T2bEntry {
    pub(crate) crc32: u32,
    pub(crate) entry_count: u8,
    pub(crate) entry_types: Vec<T2bValueType>,
    pub(crate) entry_values: Vec<i64>,
}

fn read_entries(binary_reader: &mut BinaryReader, entry_count: u32, value_length: ValueLength) -> Vec<T2bEntry> {
    let mut result = Vec::with_capacity(entry_count as usize);

    for _ in 0..entry_count {
        let crc32 = binary_reader.read_u32();
        let entry_count = binary_reader.read_byte();
        let entry_types = read_entry_types(binary_reader, entry_count as i32);
        let entry_values = read_entry_values(binary_reader, &entry_types, value_length);

        result.push(T2bEntry {
            crc32,
            entry_count,
            entry_types,
            entry_values
        });
    }

    result
}

fn read_entry_values(binary_reader: &mut BinaryReader, types: &Vec<T2bValueType>, value_length: ValueLength) -> Vec<i64> {
    let mut values = vec![0i64; types.len()];

    for j in 0..types.len() {
        match value_length {
            ValueLength::Int => values[j] = binary_reader.read_i32() as i64,
            ValueLength::Long => values[j] = binary_reader.read_i64(),
        }
    }

    values
}