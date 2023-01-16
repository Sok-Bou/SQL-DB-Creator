mod sql_db_creator;
mod secure;

use secure::Credentials;
use sql_db_creator::{ DBType, Config, setup };

fn main() {

    let credentials = Credentials::new();

    let config = Config {
        user: credentials.user,
        password: credentials.password,
        host: credentials.host
    };
    
    setup(DBType::MySql, Some(config));
}
