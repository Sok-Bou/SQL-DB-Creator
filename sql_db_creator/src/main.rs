mod database;
mod util;

use crate::database::DB;
use util::sub_paths;

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
