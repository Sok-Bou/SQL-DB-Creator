#[path = "util.rs"]
mod util;

use util::sub_paths;
use util::get_last_of_split;
use util::get_first_of_split;

pub struct Table {
    pub name: String,
    pub path: String
}

pub struct DB {
    pub name: String,
    pub tables: Vec<Table>
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
            match get_last_of_split(&table_name_path, "/") {
                Some(name) => {

                    match get_first_of_split(name, ".") {
                        Some(name) => tables.push(Table {
                            name: name.to_string(),
                            path: table_name_path
                        }),
                        None => tables.push(Table {
                            name: String::from(""),
                            path: String::from("")
                        })
                    }
                },
                None => tables.push(Table {
                    name: String::from(""),
                    path: String::from("")
                })
            };
        }

        DB {
            name: String::from(name),
            tables: tables
        }
    }
}
