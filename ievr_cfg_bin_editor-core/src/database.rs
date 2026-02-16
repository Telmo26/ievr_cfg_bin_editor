#![allow(dead_code)]

use crate::{rdbn::Rdbn, t2b::{HashType, T2b, T2bEntry, ValueLength}};

mod utils;

use serde::{Deserialize, Serialize};
pub use utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    source: DatabaseSource,
    tables: Vec<Table>,
}

impl Database {
    pub fn serialize(&self) -> String {
        let json = serde_json::to_string_pretty(&self).unwrap();
        json
    }

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
                        name: String::new(),
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
        let mut tables: Vec<Vec<T2bEntry>> = Vec::new();
        let mut index = usize::MAX;

        for entry in t2b.entries {
            if entry.name.contains("BEG") {
                if index == usize::MAX { index = 0 } // if we just started
                else { index += 1 };

                tables.push(Vec::new());
            }

            tables[index].push(entry);
        }

        let tables = tables.into_iter().map( |table| {
            let name = strip_beg(&table[0].name).to_owned(); // The first value in always the table header

            let schema = Schema {
                name: String::new(),
                fields: table[1].values.iter().map(|value| Field {
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

                rows.push(Row { 
                    name: entry.name.clone(),
                    values 
                });
            }

            Table {
                name,
                schema,
                rows,
            }
        }).collect();

        Database { source: DatabaseSource::T2B(t2b.encoding, t2b.value_length, t2b.hash_type), tables }
    }
}

impl Into<T2b> for Database {
    fn into(self) -> T2b {
        if let DatabaseSource::T2B(encoding, value_length, hash_type) = self.source {
            let mut entries: Vec<T2bEntry> = Vec::with_capacity(self.tables.iter().map(|t| t.rows.len()).sum());

            for table in self.tables {
                let len = table.rows.len() as i32; // The first row contains the number of entries

                for row in table.rows {
                    let entry = T2bEntry {
                        name: row.name,
                        values: row.values.into_iter().flat_map(|vector| {
                            vector.into_iter().map(Value::into)
                        }).collect()
                    };

                    entries.push(entry);
                }
            }

            T2b {
                entries,
                encoding,
                value_length,
                hash_type
            }

        } else {
            panic!("Trying to convert a RDBN file to T2B format");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseSource {
    RDBN,
    T2B(i16, ValueLength, HashType)
}

fn strip_beg(name: &str) -> &str {
    match name.find("_BEG") {
        Some(idx) => &name[..idx],
        None => name,
    }
}
