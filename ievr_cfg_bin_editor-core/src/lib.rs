use memmap2::Mmap;

mod rdbn;
mod t2b;
mod database;
mod common;

use std::{fs::File, path::PathBuf};

use crate::{
    database::Database,
    rdbn::Rdbn, t2b::T2b
};

pub fn parse_database(file_path: &PathBuf) -> std::io::Result<Database> {
    let file = File::open(file_path)?;

    let mmap = unsafe { Mmap::map(&file)? };

    match Rdbn::read(&mmap) {
        Some(rdbn) => Ok(rdbn.into()),
        None => match T2b::read(&mmap) {
            Some(t2b) => Ok(t2b.into()),
            None => panic!("Unable to detect file format")
        }
    }
}