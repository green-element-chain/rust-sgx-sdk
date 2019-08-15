use crate::beans::student::Student;
use std::prelude::v1::*;

use sqlite3::access;
use sqlite3::access::flags::Flags;
use sqlite3::{
    Access, DatabaseConnection, QueryFold, ResultRowAccess, SqliteResult, StatementUpdate,
};

pub fn create_student_table(conn: &mut DatabaseConnection) {
    println!("table not existed!");
    println!("crete student table");
    conn.exec(
        "CREATE TABLE student (
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

pub fn insert_bench_student(conn: &mut DatabaseConnection) {
    for (_i, j) in (0..10).enumerate() {
        let student = Student {
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
                "INSERT INTO student (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
                           VALUES ($1, $2, $3,$4, $5, $6,$7, $8,$9)",
            )
            .unwrap();
        let changes = tx
            .update(&[
                &student.id,
                &student.street,
                &student.city,
                &student.sendstatus,
                &student.datatype,
                &student.ops,
                &student.age,
                &student.clientid,
                &student.indexid,
            ])
            .unwrap();
        assert_eq!(changes, 1);
    }
    println!("insert bench data success");
}

pub fn select_student_sum(conn: &mut DatabaseConnection) {
    //select student sum(clientid)

    println!("SELECT sum(clientid) FROM student");
    let mut stmt2 = conn.prepare("SELECT sum(clientid) FROM student").unwrap();
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

pub fn select_student_list(
    conn: &mut DatabaseConnection,
    existed: bool,
) -> SqliteResult<Vec<Student>> {
    //    select student
    let mut stmt = conn.prepare("SELECT * FROM student")?;

    let snoc = |x, mut xs: Vec<_>| {
        xs.push(x);
        xs
    };

    let ppl = stmt.query_fold(&[], vec![], |row, ppl| {
        Ok(snoc(
            Student {
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
