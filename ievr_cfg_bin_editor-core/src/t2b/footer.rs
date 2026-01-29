use crate::common::binary_reader::BinaryReader;

pub struct T2bFooter {
    pub(super) magic: u32,
    _unk1: i16,
    pub(super) encoding: i16,
    _unk2: i16,
}

impl T2bFooter {
    pub fn read(binary_reader: &mut BinaryReader) -> T2bFooter {
        let magic = binary_reader.read_u32();
        let _unk1 = binary_reader.read_i16();
        let encoding = binary_reader.read_i16();
        let _unk2 = binary_reader.read_i16();

        T2bFooter { magic, _unk1, encoding, _unk2 }
    }
}