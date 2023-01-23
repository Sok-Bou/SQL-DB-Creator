use crate::util::{ sub_paths, get_last_of_split, get_first_of_split };

use std::fs;
use std::collections::HashMap;
use serde_json::Value;

pub struct Table {
    pub name: String,
    pub path: String,
    pub scheme: HashMap<String, String>,
    pub data: Vec<HashMap<String, Value>>
}

pub struct DB {
    pub name: String,
    pub tables: Vec<Table>
}

fn create_scheme(content: &str) -> HashMap<String, String> {
    let mut scheme: HashMap<String, String> = HashMap::new();

    let value_hashmap: HashMap<String, Value> = serde_json::from_str(content).unwrap();

    for (key, value) in value_hashmap {
        if key == "scheme" {
            match value {
                Value::Object(obj) => {
                    for i in &obj {
                        scheme.insert(i.0.to_string(), i.1.to_string());
                    }               
                },
                _ => ()
            }
        }
    }

    scheme
}

fn create_data(content: &str) -> Vec<HashMap<String, Value>> {
    let mut data: Vec<HashMap<String, Value>> = Vec::new();

    let value_hashmap: HashMap<String, Value> = serde_json::from_str(content).unwrap();

    for (key, value) in value_hashmap {
        if key == "data" {
            match value {
                Value::Array(array) => {
                    for a in array {    
                        data.push(serde_json::from_value(a).unwrap());
                    }
                },
                _ => ()
            }
        }
    }

    data
}

impl DB {
    pub fn new(db_name_path: &str) -> Self {

        let name = match get_last_of_split(db_name_path, "/") {
            Some(name) => name,
            None => ""
        };

        let mut tables: Vec::<Table> = Vec::new();

        let table_name_paths = match sub_paths(&db_name_path) {
            Ok(paths) => paths,
            Err(_) => return DB {
                name: String::from(""),
                tables: Vec::new()
            }
        };

        for table_name_path in table_name_paths {

            let mut scheme: HashMap<String, String> = HashMap::new();
            let mut data: Vec<HashMap<String, Value>> = Vec::new();
            match fs::read_to_string(&table_name_path) {
                Ok(content) => {
                    //println!("{content}");
                    scheme = create_scheme(&content);
                    data = create_data(&content);
                },
                Err(e) => println!("{e}")            
            }
            
            match get_last_of_split(&table_name_path, "/") {
                Some(name) => {

                    match get_first_of_split(name, ".") {
                        Some(name) => tables.push(Table {
                            name: name.to_string(),
                            path: table_name_path,
                            scheme: scheme,
                            data: data
                        }),
                        None => tables.push(Table {
                            name: String::from(""),
                            path: String::from(""),
                            scheme: scheme,
                            data: data
                        })
                    }
                },
                None => tables.push(Table {
                    name: String::from(""),
                    path: String::from(""),
                    scheme: scheme,
                    data: data
                })
            };
        }

        DB {
            name: String::from(name),
            tables: tables
        }
    }
}
