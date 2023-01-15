use std::fs;
use std::io::Error;

fn sub_paths(dir_path: &str) -> Result<Vec<String>, Error> {

    let paths = match fs::read_dir(dir_path) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(e);
        }
    };

    let mut sub_paths: Vec<String> = Vec::new();

    for path_result in paths {
        let path = match path_result {
            Ok(path) => path,
            Err(e) => {
                return Err(e);
            }
        };
        sub_paths.push(path.path().display().to_string());
    }

    Ok(sub_paths)
}

fn get_last_of_split<'a>(text: &'a str, seperator: &'a str) -> Option<&'a str> {
    let split = text.split(seperator);
    let parts: Vec<&str> = split.collect();

    if parts.len() > 0 {
        let part = parts.last();
        match part {
            Some(name) => return Some(name),
            None => return None
        }
    }

    None
}

struct DB {
    name: String,
    tables: Vec<String>
}

impl DB {
    fn new(db_name_path: &str) -> Self {

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

fn main() {

    let db_name_paths = match sub_paths("./src/db/") {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{e}");
        }
    };

    let mut dbs: Vec<DB> = Vec::new();

    for db_name_path in db_name_paths {
        dbs.push(DB::new(&db_name_path))
    }

    for db in dbs {
        println!("{}", db.name);
        for table in db.tables {
            println!("{}", table);
        }
    }
}
