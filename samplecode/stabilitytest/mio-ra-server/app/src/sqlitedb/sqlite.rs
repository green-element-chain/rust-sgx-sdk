use sgx_types::*;
use std::io::{self, BufReader, Read, Write};
use std::slice;

use crate::sqlitedb::opening;
use sqlite3::{DatabaseConnection, SqliteResult};
use sqlitedb::sqlops;

pub fn start_db(existed: uint8_t) -> SqliteResult<DatabaseConnection> {
    // A sample &'static string
    let rust_raw_string = "This is a out-Enclave ";
    // An array
    println!("{}", &rust_raw_string);

    let mut conn;
    match sqlops::get_database_conn() {
        Ok(x) => {
            conn = x;
            println!("sqlitedb opening test:");
//            opening::base_ops(&mut conn, existed);
            Ok(conn)
        }
        _ => {
            panic!("connect database failed");
        }
    }
}
