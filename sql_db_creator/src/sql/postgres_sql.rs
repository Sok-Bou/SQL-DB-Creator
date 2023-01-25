use crate::db::{ DBs, DB, Table };
use crate::util::reduce_str;
use crate::Config;

use std::collections::HashMap;
use futures::executor::block_on;
use serde_json::Value;
use sqlx::postgres::{ PgPool, PgQueryResult };
use sqlx::{Pool, Postgres, Error };

async fn create_connection(config: &Config) -> Result<Pool<Postgres>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("postgres://{user}:{password}@{host}");

    PgPool::connect(&url).await
}

async fn create_db(name: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    let query = format!("CREATE DATABASE {name}");
    sqlx::query(&query).execute(pool).await
}

async fn drop_db(name: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    let query = format!("DROP DATABASE IF EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

async fn create_pool_for_db(config: &Config, db_name: &str) -> Result<Pool<Postgres>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("postgres://{user}:{password}@{host}/{db_name}");

    PgPool::connect(&url).await
}

fn create_pools_for_dbs<'a>(config: &Config, dbs: &'a DBs, pool: &Pool<Postgres>) -> Vec::<(Pool<Postgres>, &'a DB)> {
    let mut pools: Vec::<(Pool<Postgres>, &DB)> = Vec::new();

    let dbs = &dbs.dbs;
    for db in dbs {
        match block_on(create_db(&db.name, &pool)) {
            Ok(_) => {
                println!("New Database with name \"{}\" created", &db.name);
                match block_on(create_pool_for_db(config, &db.name)) {
                    Ok(pool) => {
                        pools.push((pool, db));
                    },
                    Err(e) => {
                        println!("Error: {e}");
                        println!("Something went wron by trying to create the pool for the \"{}\" database", &db.name);
                    }
                }
            },
            Err(e) => {
                println!("Error: {e}");
                println!("Database with name \"{}\" couldn't be created", &db.name);
            }
        }
    }

    pools
}

async fn create_table(pool: &Pool<Postgres>, table: &Table) -> Result<PgQueryResult, Error> {
    let mut query = String::from("CREATE TABLE ");
    query.push_str(&table.name);
    query.push_str(" (");

    let schema = &table.schema;
    for (key, value) in schema {
        let value_new = &value[1..value.len() - 1];

        let line = format!("{} {}, ", key, value_new);
        query.push_str(&line);
    }

    let query_str = &query;
    let query_str_new = &query_str[0..query_str.len() - 2];
    
    let mut query = String::from(query_str_new);

    query.push_str(");");

    sqlx::query(&query).execute(pool).await
}

async fn create_table_data_set(pool: &Pool<Postgres>, table: &Table, data_set: &HashMap<String, Value>) -> Result<PgQueryResult, Error> {
    let mut column_names: Vec<&str> = Vec::new();
    let mut query = String::from("INSERT INTO ");
    query.push_str(&table.name);
    query.push_str(" (");

    let schema = &table.schema;
    for (s_key, _) in schema {
        for (d_key, _) in data_set {
            if s_key == d_key {
                let line = format!("{}, ", s_key);
                query.push_str(&line);
                column_names.push(&s_key);
            }
        }
    }
    
    let mut query = reduce_str(&query, 0, 2);

    query.push_str(") VALUES (");

    for name in &column_names {
        for (key, value) in data_set {
            if key == name {
                let mut value_new_string = String::from("");

                match value {
                    Value::Null => println!("Null"),
                    Value::Bool(b) => {
                        value_new_string.push_str(&b.to_string());
                    },
                    Value::Number(number) => {
                        value_new_string.push_str(&number.to_string());
                    },
                    Value::String(string) => {
                        value_new_string.push('\'');
                        value_new_string.push_str(string);
                        value_new_string.push('\'');
                    },
                    Value::Array(value) => println!("value: {:?}", value),
                    Value::Object(obj) => println!("obj: {:?}", obj)
                }

                if value_new_string != "" {
                    let line = format!("{}, ", value_new_string);
                    query.push_str(&line);
                }
            }
        }
    }

    let mut query = reduce_str(&query, 0, 2);
    query.push_str(");");

    sqlx::query(&query).execute(pool).await
}

pub fn setup_progres_sql(config: Config) {
    let dbs = DBs::new();
    let connection_pool_future_result = create_connection(&config);

    match block_on(connection_pool_future_result) {
        Ok(connection_pool) => {

            for db in &dbs.dbs {
                if let Err(_) = block_on(drop_db(&db.name, &connection_pool)) {
                    println!("Database \"{}\" could NOT be dropped. Probably the json has not the write format.", &db.name);
                } else {
                    println!("Database \"{}\" dropped if existed", &db.name);
                }
            }
            
            let pools = create_pools_for_dbs(&config, &dbs, &connection_pool);
            for (pool, db) in pools {

                let db_name = &db.name;
                let tables = &db.tables;

                for table in tables {
                    
                    let table_name = &table.name;
                    let table_result = create_table(&pool, &table);

                    if let Err(_) = block_on(table_result) {
                        println!("Table with name \"{}\" could NOT be created. Probably the json has not the write format.", table_name);
                    } else {
                        println!("New Table with name \"{}\" created in Database \"{}\".", table_name, db_name);

                        let min_size: usize = 0;
                        if &table.data.len() > &min_size {

                            let mut counter = 1;
                            for data in &table.data {
                                let data_set_result = create_table_data_set(&pool, &table, &data);

                                if let Err(_) = block_on(data_set_result) {
                                    println!("Table \"{}\" of Database \"{}\" could NOT be filled with a dataset. Probably the json has not the write format.", table_name, db_name);
                                } else {
                                    println!("Table \"{}\" of Database \"{}\" successfully filled with dataset Nr. {counter}", table_name, db_name);
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
            }
        },
        Err(_) => println!("One or mor pools couldn't be created. Probably the login credantials are wrong.")
    }
}
