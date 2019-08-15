use crate::beans::teacher::Teacher;
use std::prelude::v1::*;

use sqlite3::access;
use sqlite3::access::flags::Flags;
use sqlite3::{
    Access, DatabaseConnection, QueryFold, ResultRowAccess, SqliteResult, StatementUpdate,
};

pub fn get_database_conn<A: Access>(access: A) -> SqliteResult<DatabaseConnection> {
    let mut conn = DatabaseConnection::new(access)?;
    Ok(conn)
}
