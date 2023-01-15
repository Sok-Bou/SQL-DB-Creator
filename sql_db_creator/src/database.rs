use crate::util::sub_paths;
use crate::util::get_last_of_split;

pub struct DB {
    pub name: String,
    pub tables: Vec<String>
}

impl DB {
    pub fn new(db_name_path: &str) -> Self {

        let name = match get_last_of_split(db_name_path, "/") {
            Some(name) => name,
            None => ""
        };

        let mut tables: Vec::<String> = Vec::new();

        let table_name_paths = match sub_paths(&db_name_path) {
            Ok(paths) => paths,
            Err(_) => return DB {
                name: String::from(""),
                tables: Vec::new()
            }
        };

        for table_name_path in table_name_paths {
            match get_last_of_split(&table_name_path, "/") {
                Some(name) => tables.push(name.to_string()),
                None => tables.push(String::from(""))
            };
        }

        DB {
            name: String::from(name),
            tables: tables
        }
    }
}
