mod rdbn;
mod t2b;
mod database;
mod common;

use crate::{
    rdbn::Rdbn, t2b::T2b
};

pub use database::{
    Database, Value, Table, Row
};

pub fn parse_database(file: &[u8]) -> std::io::Result<Database> {
    match Rdbn::read(file) {
        Some(rdbn) => Ok(rdbn.into()),
        None => match T2b::read(file) {
            Some(t2b) => Ok(t2b.into()),
            None => panic!("Unable to detect file format")
        }
    }
}