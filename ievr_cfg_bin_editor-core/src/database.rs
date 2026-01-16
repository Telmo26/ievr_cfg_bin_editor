#![allow(dead_code)]

use crate::rdbn::Rdbn;

mod utils;

use utils::*;

#[derive(Debug)]
pub struct Database {
    source: DatabaseSource,
    tables: Vec<Table>,
}

impl Database {
    pub fn tables(&self) -> &Vec<Table> {
        &self.tables
    }

    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.name == name)
    }

    pub fn table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|table| table.name == name)
    }
}

impl From<Rdbn> for Database {
    fn from(rdbn: Rdbn) -> Self {
        let tables = rdbn.lists.iter().map(|list| { // For each list
            let schema = &rdbn.types[list.type_index];

            Table {
                name: list.name.clone(),
                schema: Schema {
                    fields: schema.fields.iter().map(|f| Field {
                        name: f.name.clone(),
                        value_type: ValueType::Rdbn(f.field_type),
                        count: f.count as usize,
                    }).collect(),
                },
                rows: list.values.iter().map(|row| {
                    Row {
                        values: row.iter().flat_map(|field| {
                            field.iter().map(Value::from)
                        }).collect(),
                    }
                }).collect(),
            }
        }).collect();

        Database { source: DatabaseSource::RDBN, tables }
    }
}

#[derive(Debug)]
pub enum DatabaseSource {
    RDBN,
    T2B
}

