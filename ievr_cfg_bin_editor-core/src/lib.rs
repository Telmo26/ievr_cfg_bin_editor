use memmap2::Mmap;

mod rdbn;
mod t2b;

use std::{fs::File, path::PathBuf};

use rdbn::Rdbn;

pub fn parse_rdbn(file_path: &PathBuf) -> std::io::Result<Rdbn> {
    let file = File::open(file_path)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let rdbn = Rdbn::read(mmap).unwrap();
    Ok(rdbn)
}