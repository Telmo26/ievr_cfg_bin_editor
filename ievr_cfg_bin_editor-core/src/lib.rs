mod rdbn;
mod t2b;
mod database;
mod common;

pub use crate::{
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

pub fn test_t2b_writing(file: &[u8]) {
    if let Some(t2b) = T2b::read(file) {
        let mut database: Database = t2b.into();

        for table in database.tables() {
            println!("{}", table.name());
        }

        let table = database.table_mut("CHARA_PARAM_INFO_LIST").unwrap();

        let rows = table.rows_mut();

        let row = &mut rows[0];

        row.values[1][0] = Value::Float(1.1);

        let new_t2b: T2b = database.into();

        T2b::write(new_t2b);
    }
}