use memmap2::Mmap;

mod rdbn;
mod t2b;
mod database;
mod common;

use std::{fs::File, path::PathBuf};

use crate::{
    database::Database,
    rdbn::Rdbn
};

pub fn parse_database(file_path: &PathBuf) -> std::io::Result<Database> {
    let file = File::open(file_path)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let rdbn = Rdbn::read(mmap).unwrap();
    Ok(rdbn.into())
}