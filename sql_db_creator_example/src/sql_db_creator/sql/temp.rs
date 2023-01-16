pub trait SQL {
    fn db_url(db_name: Option<&str>) -> String;
    async fn create_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error>;
    async fn create_pool(db_name: Option<&str>) -> Result<Pool<MySql>, Error>;
    fn create_pools(dbs: &Vec<DB>, pool: &Pool<MySql>) -> Vec::<Pool<MySql>>;
}

pub struct Credentials {
    pub user: String,
    pub password: String,
    pub host: String
}

pub struct MySql {
    pub creadentials: Credentials,
}

pub struct PostgreSQL {
    pub creadentials: Credentials,
}

pub struct SQLight {

}

impl MySql for SQL {
    async fn create_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
        let query = format!("CREATE DATABASE IF NOT EXISTS {name}");
        sqlx::query(&query).execute(pool).await
    }
    
    async fn create_pool(db_name: Option<&str>) -> Result<Pool<MySql>, Error> {
        let url = db_url(db_name);
    
        MySqlPool::connect(&url).await
    }
    
    fn create_pools(dbs: &Vec<DB>, pool: &Pool<MySql>) -> Vec::<Pool<MySql>> {
        let mut pools: Vec::<Pool<MySql>>  = Vec::new();
    
        for db in dbs {
            match block_on(create_db(&db.name, &pool)) {
                Ok(_) => {
                    println!("Database created: {}", &db.name);
    
                    match block_on(create_pool(Some(&db.name))) {
                        Ok(pool) => pools.push(pool),
                        Err(e) => println!("Something went wron by trying to create a pool: {e}")
                    }
                },
                Err(e) => println!("Database could't be created: {e}")
            }
        }
    
        pools
    }

    fn db_url(db_name: Option<&str>) -> String {
        let credentials = Credentials::new();
    
        let user = credentials.user;
        let password = credentials.password;
        let host = credentials.host;
    
        match db_name {
            Some(db_name) => return format!("mysql://{user}:{password}@{host}/{db_name}"),
            None => return format!("mysql://{user}:{password}@{host}")
        }
    }
}