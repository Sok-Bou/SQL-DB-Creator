mod database;
mod util;
mod secure;

use crate::database::DB;
use util::sub_paths;
use secure::Credentials;
use futures::executor::block_on;

use sqlx::mysql::MySqlPool;
use sqlx::Pool;
use sqlx::MySql;
use sqlx::Error;
use sqlx::mysql::MySqlQueryResult;

// use sqlx::postgres::PgPoolOptions;

async fn connect() -> Result<Pool<MySql>, Error> {
    let credentials = Credentials::new();

    let user = credentials.user;
    let password = credentials.password;
    let host = credentials.host;

    println!("User: {user}, Password: {password}, Host: {host}");

    let url = format!("mysql://{user}:{password}@{host}");

    MySqlPool::connect(&url).await
}

async fn create_db(pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    sqlx::query("CREATE DATABASE IF NOT EXISTS test").execute(pool).await
}

fn main() {

    let pool_future = connect();

    match block_on(pool_future) {
        Ok(pool) => {
            match block_on(create_db(&pool)) {
                Ok(query) => {
                    println!("Database created: {:?}", query);
                },
                Err(e) => println!("Database could't be created: {e}")
            }
        },
        Err(e) => println!("Pool could't be created: {e}")
    }


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
            println!("{}", table.name);
            println!("{}", table.path);
        }
    }
}
