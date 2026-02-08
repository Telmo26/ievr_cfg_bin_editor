use crc_fast::{CrcParams, checksum_with_params};

pub(crate) mod binary_reader;
pub(crate) mod binary_writer;

pub(crate) fn compute_crc32_standard(data: &[u8]) -> u32 {
    let params = CrcParams::new(
        "CRC/STANDARD", 
        32, 
        0x04C11DB7, 
        0xFFFF_FFFF, 
        true, 
        0xFFFF_FFFF, 
        0
    );

    checksum_with_params(params, data) as u32
}

pub(crate) fn compute_crc32_jam(data: &[u8]) -> u32 {
    let params = CrcParams::new(
        "CRC/JAM", 
        32, 
        0x04C11DB7, 
        0xFFFF_FFFF, 
        true, 
        0x0000_0000, 
        0
    );

    checksum_with_params(params, data) as u32
}