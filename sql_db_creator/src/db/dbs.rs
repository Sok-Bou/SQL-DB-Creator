use crate::db::db::DB;
use crate::util::sub_paths;

pub struct DBs {
    pub dbs: Vec<DB>,
}

impl DBs {
    pub fn new() -> Self {
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

        Self {
            dbs: dbs
        }
    }

    #[allow(dead_code)]
    pub fn print_db(&self) {
        let dbs = &self.dbs;
        for db in dbs {
            println!("{}", db.name);
    
            let tables = &db.tables;
    
            for table in tables {
                println!("{}", table.name);
                println!("{}", table.path);

                let scheme = &table.scheme;
                for (key, value) in scheme {
                    println!("    {key}, {value}");
                }

                let data = &table.data;
                for data_set in data {
                    println!(" ------ ");
                    for (key, value) in data_set {
                        println!("        {key}, {value}");
                    }
                }
            }
        }
    }
}
