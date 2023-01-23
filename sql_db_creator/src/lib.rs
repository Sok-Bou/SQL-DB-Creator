mod db;
mod sql;
mod util;

use sql::setup_my_sql;
use sql::setup_progres_sql;

/// Holds the login creadentials and the host for the db.
/// 
/// The data are been used to setup the url for the db connection.
/// 
/// For MySql: mysql://{user}:{password}@{host}
/// 
/// For PostgresSql: postgres://{user}:{password}@{host}
pub struct Config {
    pub user: String,
    pub password: String,
    pub host: String
}

/// The db type of that the user wants to create a db.
pub enum DBType {
    MySql,
    PostgresSql
}

/// Initiates the db creation.
pub fn setup(db_type: DBType, config: Config) {
    match db_type {
        DBType::MySql => setup_my_sql(config),
        DBType::PostgresSql => setup_progres_sql(config)
    }
}
