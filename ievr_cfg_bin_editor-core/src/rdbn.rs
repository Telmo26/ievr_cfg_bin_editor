use std::collections::HashMap;

use memmap2::Mmap;

mod header;
mod root_entry;
mod type_entry;
mod field_entry;

mod field_type;
mod field_type_category;
mod declarations;
mod list_entry;

use self::{
    header::RdbnHeader,
    root_entry::RdbnRootEntry,
    type_entry::RdbnTypeEntry,
    field_entry::RdbnFieldEntry,
    declarations::{RdbnFieldDeclaration, RdbnTypeDeclaration},
    field_type_category::RdbnFieldTypeCategory,
    list_entry::RdbnListEntry,
};

pub use field_type::RdbnFieldType;
pub use list_entry::RdbnValue;

use super::common::binary_reader::BinaryReader;

const MINIMUM_SIZE: usize = 0x3C;
const RDBN_HEADER: u32 = const { u32::from_le_bytes(*b"RDBN") };

pub struct Rdbn {
    pub types: Vec<RdbnTypeDeclaration>,
    pub lists: Vec<RdbnListEntry>,
}

impl Rdbn {
    pub fn read(file: Mmap) -> Option<Rdbn> {
        if file.len() < MINIMUM_SIZE {
            return None
        }

        let mut binary_reader = BinaryReader::new(file);

        let header = RdbnHeader::new(&mut binary_reader);

        if header.magic != RDBN_HEADER {
            return None;
        }

        let data_offset = header.data_offset << 2;

        // Read root entries
        let position = (header.root_offset << 2) + data_offset; debug_assert!(position > 0);
        binary_reader.set_position(position as usize);
        let root_entries = Self::read_root_entries(&mut binary_reader, header.root_count);

        // Read type entries
        let position = (header.type_offset << 2) + data_offset; debug_assert!(position > 0);
        binary_reader.set_position(position as usize);
        let type_entries = Self::read_type_entries(&mut binary_reader, header.type_count);

        // Read field entries
        let position = (header.field_offset << 2) + data_offset; debug_assert!(position > 0);
        binary_reader.set_position(position as usize);
        let field_entries = Self::read_field_entries(&mut binary_reader, header.field_count);

        let hash_offset = (header.string_hash_offset << 2) + data_offset;
        let offset_offset = (header.string_offsets_offset << 2) + data_offset;
        let string_offset = header.string_offset + data_offset as i32;

        if let Some(string_lookup) = Self::read_strings(&mut binary_reader, header.hash_count, hash_offset, offset_offset, string_offset) {
            let value_offset = ((header.value_offset as i32) << 2) + data_offset as i32;
            return Some(Self::create_rdbn(binary_reader, value_offset, string_offset, root_entries, type_entries, field_entries, string_lookup))
        } else {
            None
        }
    }

    fn read_root_entries(binary_reader: &mut BinaryReader, root_count: i16) -> Vec<RdbnRootEntry> {
        debug_assert!(root_count > 0);
        let count = root_count as usize;
        
        let mut result: Vec<RdbnRootEntry> = Vec::with_capacity(count);

        for _ in 0..count {
            result.push(RdbnRootEntry::new(binary_reader));
            binary_reader.skip(12); // The RdbnRootEntry is only 20 bytes long, but the entries are 0x20 = 32 aligned
        }

        result
    }

    fn read_type_entries(binary_reader: &mut BinaryReader, type_count: i16) -> Vec<RdbnTypeEntry> {
        debug_assert!(type_count > 0);
        let count = type_count as usize;

        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(RdbnTypeEntry::new(binary_reader));
            binary_reader.skip(20); // The RdbnTypeEntry is only 12 bytes long, but the entries are 0x20 = 32 aligned
        }

        result
    }

    fn read_field_entries(binary_reader: &mut BinaryReader, field_count: i16) -> Vec<RdbnFieldEntry> {
        debug_assert!(field_count > 0);
        let count = field_count as usize;

        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(RdbnFieldEntry::new(binary_reader));
            binary_reader.skip(12); // The RdbnFieldEntry is only 20 bytes long, but the entries are 0x20 = 32 aligned
        }

        result
    }

    fn read_strings(binary_reader: &mut BinaryReader, hash_count: i16, hash_offset: i16, offset_offset: i16, string_offset: i32) -> Option<HashMap<u32, String>> {
        debug_assert!(hash_count > 0);

        let count = hash_count as usize;
        let mut hashes = Vec::with_capacity(count);
        let mut offsets = Vec::with_capacity(count);

        debug_assert!(hash_offset > 0);
        binary_reader.set_position(hash_offset as usize);

        for _ in 0..count {
            hashes.push(binary_reader.read_u32());
        }

        debug_assert!(offset_offset > 0);
        binary_reader.set_position(offset_offset as usize);

        for _ in 0..count {
            offsets.push(binary_reader.read_i32());
        }

        let mut result = HashMap::with_capacity(count);

        for i in 0..count {
            if (string_offset + offsets[i]) as usize > binary_reader.file_size() {
                return None
            }

            debug_assert!(string_offset + offsets[i] > 0);
            binary_reader.set_position((string_offset + offsets[i]) as usize);
            result.insert(hashes[i], Self::read_string(binary_reader));
        }

        Some(result)
    }

    fn read_string(binary_reader: &mut BinaryReader) -> String {
        let mut result = Vec::new();

        let mut byte = binary_reader.read_byte();
        while byte != 0 {
            result.push(byte);
            byte = binary_reader.read_byte();
        }

        String::from_utf8(result).unwrap()
    }

    fn create_rdbn(mut binary_reader: BinaryReader, value_offset: i32, string_offset: i32, root_entries: Vec<RdbnRootEntry>, type_entries: Vec<RdbnTypeEntry>, field_entries: Vec<RdbnFieldEntry>, string_lookup: HashMap<u32, String>) -> Rdbn {
        let mut type_declarations = Vec::with_capacity(type_entries.len());
        for type_entry in &type_entries {
            debug_assert!(type_entry.field_count > 0);
            let mut field_declarations = Vec::with_capacity(type_entry.field_count as usize);

            for j in 0..type_entry.field_count as usize {
                debug_assert!(type_entry.field_index >= 0);
                let field_entry = &field_entries[type_entry.field_index as usize + j];

                field_declarations.push(RdbnFieldDeclaration {
                    name: string_lookup[&field_entry.name_hash].clone(),
                    count: field_entry.value_count,
                    size: field_entry.value_size,
                    field_type: RdbnFieldType::try_from(field_entry.r#type).unwrap(),
                    field_type_category: RdbnFieldTypeCategory::try_from(field_entry.type_category).unwrap(),
                });
            }

            type_declarations.push(RdbnTypeDeclaration {
                name: string_lookup[&type_entry.name_hash].clone(),
                unk_hash: type_entry.unk1,
                fields: field_declarations,
            });
        }

        let mut distinct_types = Vec::<RdbnTypeDeclaration>::new();
        let mut lookup = HashMap::<RdbnTypeDeclaration, usize>::new();

        for ty in &type_declarations {
            if lookup.get(&ty).is_none() {
                let idx = distinct_types.len();
                distinct_types.push(ty.clone());
                lookup.insert(ty.clone(), idx);
            }
        }

        let mut lists = Vec::with_capacity(root_entries.len());

        for root_entry in &root_entries {
            debug_assert!(root_entry.value_count > 0);
            let mut list_values = Vec::with_capacity(root_entry.value_count as usize);

            let root_value_offset = value_offset as i32 + root_entry.value_offset;

            for j in 0..root_entry.value_count {
                let type_entry = &type_entries[root_entry.type_index as usize];

                list_values.push(Vec::with_capacity(type_entry.field_count as usize));

                let type_value_offset = root_value_offset + j * root_entry.value_size;

                for h in 0..type_entry.field_count {
                    let field_entry = &field_entries[(type_entry.field_index + h) as usize];

                    list_values[j as usize].push(Vec::with_capacity(field_entry.value_count as usize));

                    binary_reader.set_position((type_value_offset + field_entry.value_offset) as usize);

                    for _ in 0..field_entry.value_count {
                        match field_entry.r#type {
                            // Ability Data
                            0..3 => list_values[j as usize][h as usize].push(
                                RdbnValue::Bytes(binary_reader.read_bytes(field_entry.value_size as usize).to_vec())
                            ),
                            3 => list_values[j as usize][h as usize].push(
                                RdbnValue::Bool(binary_reader.read_bool())
                            ),
                            4 => list_values[j as usize][h as usize].push(
                                RdbnValue::Byte(binary_reader.read_byte())
                            ),
                            5 | 9 => list_values[j as usize][h as usize].push(
                                RdbnValue::Short(binary_reader.read_i16())
                            ),
                            6 | 10 => list_values[j as usize][h as usize].push(
                                RdbnValue::Int(binary_reader.read_i32())
                            ),
                            0xD => list_values[j as usize][h as usize].push(
                                RdbnValue::Float(binary_reader.read_f32())
                            ),
                            0xF => list_values[j as usize][h as usize].push(
                                RdbnValue::Uint(binary_reader.read_u32())
                            ),
                            0x12 | 0x13 => list_values[j as usize][h as usize].push(
                                RdbnValue::Float4([binary_reader.read_f32(), binary_reader.read_f32(), binary_reader.read_f32(), binary_reader.read_f32()])
                            ),
                            0x14 => {
                                let condition_value = binary_reader.read_u32();

                                if string_offset as usize + condition_value as usize >= binary_reader.file_size() {
                                    list_values[j as usize][h as usize].push(
                                        RdbnValue::Uint(condition_value)
                                    );
                                } else {
                                    binary_reader.set_position(string_offset as usize + condition_value as usize);
                                    list_values[j as usize][h as usize].push(
                                        RdbnValue::String(Self::read_string(&mut binary_reader))
                                    );
                                }
                            }
                            0x15 => list_values[j as usize][h as usize].push(
                                RdbnValue::Short2([binary_reader.read_i16(), binary_reader.read_i16()])
                            ),
                            _ => panic!("Invalid field type {:X}", field_entry.r#type)
                        }
                    }
                }
            }

            lists.push(RdbnListEntry {
                name: string_lookup[&root_entry.name_hash].clone(),
                type_index: lookup[&type_declarations[root_entry.type_index as usize]],
                values: list_values,
            });
        }

        return Rdbn {
            types: distinct_types,
            lists,
        }
    }
}