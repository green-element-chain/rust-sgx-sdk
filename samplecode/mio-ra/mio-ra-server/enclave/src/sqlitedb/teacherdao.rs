use crate::beans::teacher::Teacher;
use std::prelude::v1::*;

use sqlite3::access;
use sqlite3::access::flags::Flags;
use sqlite3::{
    Access, DatabaseConnection, QueryFold, ResultRowAccess, SqliteResult, StatementUpdate,
};

pub fn create_teacher_table(conn: &mut DatabaseConnection) {
    println!("table not existed!");
    println!("crete teacher table");
    conn.exec(
        "CREATE TABLE teacher (
                 id              SERIAL PRIMARY KEY,
                 street          VARCHAR NOT NULL,
                 city            VARCHAR NOT NULL,
                 sendstatus      VARCHAR NOT NULL,
                 datatype        VARCHAR NOT NULL,
                 ops             VARCHAR NOT NULL,
                 age             integer,
                 clientid        integer,
                 indexid         integer
               )",
    )
    .unwrap();
}

pub fn insert_bench_teacher(conn: &mut DatabaseConnection) {
    for (_i, j) in (0..10).enumerate() {
        let teacher = Teacher {
            id: j,
            street: "streett".to_string(),
            city: "cityt".to_string(),
            sendstatus: "sendstatust".to_string(),
            datatype: "datatypet".to_string(),
            ops: "insert".to_string(),
            age: j,
            clientid: 10000,
            indexid: j,
        };

        let mut tx = conn
            .prepare(
                "INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
                           VALUES ($1, $2, $3,$4, $5, $6,$7, $8,$9)",
            )
            .unwrap();
        let changes = tx
            .update(&[
                &teacher.id,
                &teacher.street,
                &teacher.city,
                &teacher.sendstatus,
                &teacher.datatype,
                &teacher.ops,
                &teacher.age,
                &teacher.clientid,
                &teacher.indexid,
            ])
            .unwrap();
        assert_eq!(changes, 1);
    }
    println!("insert bench data success");
}

pub fn select_teacher_sum(conn: &mut DatabaseConnection) {
    //select teacher sum(clientid)

    println!("SELECT sum(clientid) FROM teacher");
    let mut stmt2 = conn.prepare("SELECT sum(clientid) FROM teacher").unwrap();
    let mut results = stmt2.execute();
    match results.step() {
        Ok(Some(ref mut row1)) => {
            let id = row1.column_int(0);
            println!("clientid sum is {}", id);
        }
        Err(oops) => panic!(oops),
        Ok(None) => panic!("where did our row go?"),
    }
}

pub fn select_teacher_list(
    conn: &mut DatabaseConnection,
    existed: bool,
) -> SqliteResult<Vec<Teacher>> {
    //    select teacher
    let mut stmt = conn.prepare("SELECT * FROM teacher")?;

    let snoc = |x, mut xs: Vec<_>| {
        xs.push(x);
        xs
    };

    let ppl = stmt.query_fold(&[], vec![], |row, ppl| {
        Ok(snoc(
            Teacher {
                id: row.get(0),
                street: row.get(1),
                city: row.get(2),
                sendstatus: row.get(3),
                datatype: row.get(4),
                ops: row.get(5),
                age: row.get(6),
                clientid: row.get(7),
                indexid: row.get(8),
            },
            ppl,
        ))
    })?;
    Ok(ppl)
}
