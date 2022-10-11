use std::ops::Index;
use std::str::from_utf8;

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://username:password@127.0.0.1:3306/database_name";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    for result in conn.query_iter("SELECT TABLE_NAME, COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS ORDER BY ORDINAL_POSITION")? {
        let result = result.unwrap();
        let table_name_result = result.index(0);

        if let Value::Bytes(table_name) = table_name_result {
            println!("{:#?}", from_utf8(table_name)?);
        }

    }

    Ok(())
}

