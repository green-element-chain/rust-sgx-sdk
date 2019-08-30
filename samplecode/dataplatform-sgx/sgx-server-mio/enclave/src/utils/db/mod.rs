//! 数据库操作的通用方法实现，可以作为一个适配层适配不同的数据库类型
use sqlite3::{access, DatabaseConnection, PreparedStatement, SqliteResult, StatementUpdate};

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use std::string::String;

#[macro_export(local_inner_macros)]
macro_rules! snoc {
    ($elem:expr; $c:expr) => {
        $c.push($elem);
        $c
    }
}

pub struct DbContext {
    conn: Rc<RefCell<DatabaseConnection>>,
    timezone: i32,
}

impl DbContext {
    pub fn new(db_file: &str, _timezone: i32) -> DbContext {
        let mut connection = DatabaseConnection::in_memory().unwrap();
        if !db_file.is_empty() {
            connection = DatabaseConnection::new(access::ByFilename {
                filename: db_file,
                flags: Default::default(),
            }).unwrap();
        }

        DbContext {
            conn: Rc::new(RefCell::new(connection)),
            timezone: _timezone,
        }
    }

    pub fn timezone_str(&self) -> String {
        format!("{:?}", chrono::FixedOffset::east(self.timezone))
    }
}

impl DbContext {
    //用于创建表操作
    pub fn exec(&self, sql: &str) {
        let result = self.conn.borrow_mut().exec(sql);
        match result {
            Ok(_) => {}
            Err(e) => {
                error!("failed to execute sql, error message: {}", e);
                error!("sql: {}", sql);
            }
        }
    }

    //执行插入、更新、删除等操作
    pub fn execute(&self, sql: &str) -> bool {
        let mut result = self.conn.borrow_mut().prepare(sql);
        match result {
            Ok(ref mut stmt) => {
                let changes = stmt.update(&[]);
                match changes {
                    Ok(_num) => { true }
                    Err(e) => {
                        error!("insert execute update failed {}", e);
                        error!("sql: {}", sql);
                        false
                    }
                }
            }
            Err(e) => {
                error!("insert execute prepare failed {}", e);
                error!("sql: {}", sql);
                false
            }
        }
    }

    pub fn query(&self, sql: &str) -> SqliteResult<PreparedStatement> {
        let result = self.conn.borrow_mut().prepare(sql);
        match result {
            Ok(stmt) => { Ok(stmt) }
            Err(e) => {
                error!("failed to prepare query sql, error message: {}", e);
                error!("sql: {}", sql);
                Err(e)
            }
        }
    }

    //查询数据库中表的数量
    pub fn table_count(&self) -> u32 {
        let mut count: u32 = 0;

        let sql = "select count(1) from sqlite_master where type = 'table'";
        let statement: SqliteResult<PreparedStatement> = self.query(sql);
        if statement.is_ok() {
            match statement.unwrap().execute().step() {
                Err(e) => {
                    error!("failed to query project_bill {}", e);
                }
                Ok(None) => {}
                Ok(Some(ref mut row)) => {
                    count = row.column_int(0) as u32
                }
            }
        }
        count
    }

    //查询表记录最后的更新时间
    pub fn get_last_update_time_local(&self, table: &str) -> Option<String> {
        let tz_str = self.timezone_str();
        let sql = format!("select datetime(max(update_at), '{}') from {}", tz_str.as_str(), table);
        info!("get last update time sql: {}", sql.as_str());

        let statement: SqliteResult<PreparedStatement> = self.query(sql.as_str());
        if statement.is_ok() {
            match statement.unwrap().execute().step() {
                Err(e) => {
                    error!("failed to execute step of query for project_bill {}", e);
                    return None;
                }
                Ok(None) => { return Some(String::from("")); }
                Ok(Some(ref mut row)) => {
                    return Some(row.column_text(0).unwrap_or(String::from("")));
                }
            }
        }
        None
    }
}