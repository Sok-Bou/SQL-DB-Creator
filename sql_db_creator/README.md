Generate sql databases by only configure the data of the databases in json files.

Check the examples after reading.
https://github.com/Sok-Bou/SQL-DB-Creator-Example

With this crate you can generate multiple mysql or postgressql databases and fill 
the (optionally) with data.

Steps:
1. In the src directory add a folder with name db. It has to be in the src directory and with the 
name db
2. Add one or more folders depending on how many databases you want. Most of the time it is one
database. The name of the folder would be the name of the database.
3. Add one or more json files. The names of the json files would be the names of the database tables.

By now you folder structure should look like this

📂  src

 ┣- 📂  db  // (1)

 ┃ ┣--- 📂 countries  // (2)   

 ┃ ┃ ┣ --- 📜  geography.json  // (3)     

 ┃ ┃ ┗ --- 📜  goverment.json  // (4)

 ┃ ┗--- 📂  flowers  // (5)

 ┃ ┃ ┣ --- 📜  infos.json  // (6)

 ┃ ┃ ┗ --- 📜  region.json  // (7)

 ┣ 📜  main.rs  // (8)


(1) Required name 'db'

(2) First database with name 'countries'

(3) Table in database 'countries' with name 'geography'

(4) Table in database 'countries' with name 'goverment'

(5) Second database with name 'flowers'

(6) Table in database 'flowers' with name 'infos'

(7) Table in database 'flowers' with name 'region'

(8) The main file

4. Now the json files should have a specific structure to generate the columns of the table. The 
structures goes like this:
(mysql example)

```json
{
    "schema": {
        "name": "VARCHAR(255)",
        "area": "INT(255)",
        "region": "VARCHAR(255)",
        "costline": "INT(255)",
        "borders": "INT(255)",
        "is_in_europe": "BIT"
    },
    "data": [
        {
            "name": "Germany",
            "area": 357021,
            "region": "Central Europe",
            "costline": 2389,
            "borders": 3714,
            "is_in_europe": true
        },
        {
            "name": "United States of America",
            "area": 9826675,
            "region": "North Americs",
            "costline": 19920,
            "borders": 12191,
            "is_in_europe": false
        }
    ]
}
```

In the schema part are the infos to create the schema of the table with column names and data types as they appear in the example above.
In the data part are the infos to fill the table with our table data.
The data section is optional. We can only generate the table. Of course in case we fill the table with data, we have to be carefull with the values.
The values should have the right type as given in the schema.

5. To generate the database run the following code in your main
(mysql example)

```rust
use sql_db_creator::{ DBType, Config, setup };

fn main() {

    let config = Config {
        user: String::from("root"),
        password: String::from("password"),
        host: String::from("localhost")
    };

    setup(DBType::MySql, config);
}
```