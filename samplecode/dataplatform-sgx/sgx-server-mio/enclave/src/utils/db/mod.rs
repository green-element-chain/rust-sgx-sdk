//! 数据库操作的通用方法实现，可以作为一个适配层适配不同的数据库类型
use sqlite3::{access, DatabaseConnection, PreparedStatement};

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

pub struct DbContext {
    conn: Rc<RefCell<DatabaseConnection>>,
}

impl DbContext {
    pub fn new(db_file: &str) -> DbContext {
        let mut connection = DatabaseConnection::in_memory().unwrap();
        if !db_file.is_empty() {
            connection = DatabaseConnection::new(access::ByFilename {
                filename: db_file,
                flags: Default::default(),
            }).unwrap();
        }

        DbContext {
            conn: Rc::new(RefCell::new(connection)),
        }
    }
}

impl DbContext {
    pub fn execute(&self, sql: &str) {
        self.conn.borrow_mut().exec(sql).unwrap()
    }

    pub fn query(&self, sql: &str) -> PreparedStatement {
        self.conn.borrow_mut().prepare(sql).unwrap()
    }
}