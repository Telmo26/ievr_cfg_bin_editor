use std::{collections::{HashMap, HashSet}, fs::File, io::Write};

use crate::{common::{binary_writer::BinaryWriter, compute_crc32_jam, compute_crc32_standard}, t2b::{HashType, T2bEntry, T2bValue, T2bValueType, ValueLength, checksum_section::T2bChecksumHeader, entry_section::T2bEntryHeader}};

use super::{
    T2b,
};

impl T2b {
    pub fn write(self, output_file: &mut File) -> Result<(), std::io::Error> {
        let mut bw = BinaryWriter::default();

        bw.set_position(0x10);

        let entry_header = write_entries(&mut bw, &self);

        bw.set_position(0);
        write_header(&mut bw, &entry_header);

        let checksum_partition_offset = (entry_header.string_data_offset + entry_header.string_data_length + 0xF) & !0xF;

        bw.set_position(checksum_partition_offset as usize + 0x10);
        let checksum_header = write_checksum_entries(&mut bw, &self);

        bw.set_position(checksum_partition_offset as usize);
        write_checksum_header(&mut bw, &checksum_header);

        bw.set_position((checksum_partition_offset + checksum_header._size) as usize);
        write_footer(&mut bw, self.encoding);

        output_file.write_all(bw.get_data())
    }
}

fn write_entries(bw: &mut BinaryWriter, t2b: &T2b) -> T2bEntryHeader {
    let entry_count = t2b.entries.len() as u32;

    let entry_length: usize = t2b.entries.iter().map(|entry| {
        4 + ((entry.values.len().div_ceil(4) + 4) & !3) + entry.values.len() * t2b.value_length as usize
    })
    .sum();

    let string_data_offset = ((0x10 + entry_length + 0xF) & !0xF) as u32;

    let mut string_offset= bw.get_position() as u32 + string_data_offset - 0x10;
    let string_offset_base = string_offset;
    
    let mut written_strings: HashMap<String, i64> = HashMap::new();
    let mut string_count = 0u32;

    for entry in &t2b.entries {
        write_entry(bw, entry, t2b.encoding, t2b.hash_type, t2b.value_length, string_offset_base, &mut written_strings, &mut string_offset, &mut string_count)
    }

    bw.write_alignment(0x10, 0xFF);

    let string_data_length = string_offset - string_data_offset;
    let string_data_count = string_count as u32;

    bw.set_position(string_offset as usize);

    bw.write_alignment(0x10, 0xFF);

    return T2bEntryHeader {
        entry_count,
        string_data_offset,
        string_data_length,
        _string_data_count: string_data_count
    }
}

fn write_entry(bw: &mut BinaryWriter, entry: &T2bEntry, encoding: i16, hash_type: HashType, 
    value_length: ValueLength, string_offset_base: u32, written_strings: &mut HashMap<String, i64>, 
    string_offset: &mut u32, string_count: &mut u32) 
{
    match hash_type {
        HashType::Crc32Standard => bw.write_u32(compute_crc32_standard(entry.name.as_bytes())),
        HashType::Crc32Jam => bw.write_u32(compute_crc32_jam(entry.name.as_bytes())),
    }

    bw.write_u8(entry.values.len() as u8);

    let mut types_written = 0;
    let mut type_buffer = 0;

    for i in 0..entry.values.len() {
        if types_written >= 4 {
            bw.write_u8(type_buffer);
            type_buffer = 0;
            types_written = 0;
        }

        type_buffer |= (entry.values[i].r#type as u8) << (i % 4 * 2);
        types_written += 1;
    }

    if types_written > 0 { bw.write_u8(type_buffer); }

    bw.write_alignment(4, 0xFF);

    for value in &entry.values {
        match value.r#type {
            T2bValueType::String => write_string(bw, 
                if let T2bValue::String(s) = &value.value { s } else { panic!() },
                 encoding, value_length, string_offset_base, written_strings, string_offset, string_count),

            T2bValueType::Integer => match value_length {
                ValueLength::Int => bw.write_i32(if let T2bValue::Integer(i) = value.value { i } else { panic!() }),
                ValueLength::Long => bw.write_i64(if let T2bValue::Long(i) = value.value { i } else { panic!() })
            }

            T2bValueType::FloatingPoint => match value_length {
                ValueLength::Int => bw.write_f32(if let T2bValue::F32(i) = value.value { i } else { panic!() }),
                ValueLength::Long => bw.write_f64(if let T2bValue::F64(i) = value.value { i } else { panic!() })
            }

            T2bValueType::Invalid => panic!("Trying to write invalid value type"),
        }
    }
}

fn write_string(bw: &mut BinaryWriter, string: &str, encoding: i16, value_length: ValueLength, string_offset_base: u32, written_strings: &mut HashMap<String, i64>, string_offset: &mut u32, string_count: &mut u32) {
    if let Some(name_offset) = written_strings.get(string) {
        write_value(bw, name_offset - string_offset_base as i64, value_length);
        return
    }

    *string_count += 1;

    write_value(bw, (*string_offset - string_offset_base) as i64, value_length);
    let entry_offset = bw.get_position();

    bw.set_position(*string_offset as usize);
    cache_strings(*string_offset as i64, string, encoding, written_strings);
    
    bw.write_string(&string, true);

    *string_offset = bw.get_position() as u32;

    bw.set_position(entry_offset);
}


fn write_value(bw: &mut BinaryWriter, value: i64, value_length: ValueLength) {
    match value_length {
        ValueLength::Int => bw.write_i32(value as i32),
        ValueLength::Long => bw.write_i64(value),
    }
}

fn cache_strings(mut position: i64, value: &str, _encoding: i16, written_strings: &mut HashMap<String, i64>) {
    for (offset, ch) in value.char_indices() {
        // Create a slice from the current offset to the end
        let suffix = &value[offset..];
        
        // Only allocate the String (heap) when inserting into the Map
        if written_strings.contains_key(suffix) { // This means that every following suffix is also in the hashmap
            break; 
        }
        written_strings.insert(suffix.to_owned(), position);

        // Update position
        position += ch.len_utf8() as i64;
    }

    // Cache the final empty string
    if !written_strings.contains_key("") {
        written_strings.insert("".to_owned(), position);
    }
}

fn write_header(bw: &mut BinaryWriter, header: &T2bEntryHeader) {
    bw.write_u32(header.entry_count);
    bw.write_u32(header.string_data_offset);
    bw.write_u32(header.string_data_length);
    bw.write_u32(header._string_data_count);
}

fn write_checksum_entries(bw: &mut BinaryWriter, t2b: &T2b) -> T2bChecksumHeader {
    let mut seen = HashSet::new();

    let names: Vec<&str> = t2b.entries
        .iter()
        .map(|e| e.name.as_str())
        .filter(|&name| seen.insert(name))
        .collect();

    let header_string_offset = ((0x10 + names.len() * 8 + 0xF) & !0xF) as u32;

    let mut string_offset = (bw.get_position() as u32 + header_string_offset - 0x10) as u32;
    let string_offset_base = string_offset;
    let mut written_strings = HashMap::new();
    let mut string_count = 0;

    for name in names {
        bw.write_u32(
            match t2b.hash_type {
                HashType::Crc32Standard => compute_crc32_standard(name.as_bytes()),
                HashType::Crc32Jam => compute_crc32_jam(name.as_bytes())
        });
        write_string(bw, name, t2b.encoding, t2b.value_length, string_offset_base, &mut written_strings, &mut string_offset, &mut string_count);
    }

    bw.write_alignment(0x10, 0xFF);

    let header = T2bChecksumHeader {
        _size: header_string_offset + (string_offset - string_offset_base + 0xF) & !0xF,
        count: string_count,
        string_offset: header_string_offset,
        string_size: string_offset - string_offset_base,
    };

    bw.set_position(string_offset as usize);
    bw.write_alignment(0x10, 0xFF);

    return header
}

fn write_checksum_header(bw: &mut BinaryWriter, header: &T2bChecksumHeader) {
    bw.write_u32(header._size);
    bw.write_u32(header.count);
    bw.write_u32(header.string_offset);
    bw.write_u32(header.string_size);
}

fn write_footer(bw: &mut BinaryWriter, encoding: i16) {
    bw.write_string("\x01t2b", false);
    bw.write_i16(0x1FE);
    bw.write_i16(encoding);
    bw.write_i16(1);

    bw.write_alignment(0x10, 0xFF);
}