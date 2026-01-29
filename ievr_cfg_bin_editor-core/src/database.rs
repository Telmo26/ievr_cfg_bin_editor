#![allow(dead_code)]

use std::collections::HashMap;

use crate::{rdbn::Rdbn, t2b::{T2b, T2bEntry}};

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
                    name: schema.name.clone(),
                    fields: schema.fields.iter().map(|f| Field {
                        name: f.name.clone(),
                        value_type: ValueType::Rdbn(f.field_type),
                        count: f.count as usize,
                    }).collect(),
                },
                rows: list.values.iter().map(|row| {
                    Row {
                        values: row.iter().map(|values| {
                            values.iter().map(Value::from).collect() // We convert every value in the database to the abstracted one
                        }).collect(),
                    }
                }).collect(),
            }
        }).collect();

        Database { source: DatabaseSource::RDBN, tables }
    }
}

impl From<T2b> for Database {
    fn from(t2b: T2b) -> Self {
        let mut t2b_iter = t2b.entries.into_iter();

        t2b_iter.next().unwrap(); // The first info is the size of the file. It is not useful since we want to group by name
        
        let mut tables: Vec<Vec<T2bEntry>> = Vec::new();

        let mut hash_map: HashMap<String, usize> = HashMap::new();
        let mut index = 0;

        for entry in t2b_iter {
            let i = hash_map.entry(entry.name.clone()).or_insert_with(|| {
                let idx = index;
                tables.push(Vec::new());
                index += 1;
                idx
            });
            tables[*i].push(entry);
        };

        let tables = tables.into_iter().map( |table| {
            let name = table[0].name.clone();

            let schema = Schema {
                name: String::new(),
                fields: table[0].values.iter().map(|value| Field {
                    name: String::new(),
                    value_type: ValueType::T2b(value.r#type),
                    count: 1,
                }).collect()
            };

            let mut rows = Vec::with_capacity(table.len());
            for entry in table {
                let values = entry.values.iter().map(|value | {
                    vec![Value::from(value)]
                }).collect();

                rows.push(Row { values });
            }

            Table {
                name,
                schema,
                rows,
            }
        }).collect();

        Database { source: DatabaseSource::T2B, tables }
    }
}

#[derive(Debug)]
pub enum DatabaseSource {
    RDBN,
    T2B
}

