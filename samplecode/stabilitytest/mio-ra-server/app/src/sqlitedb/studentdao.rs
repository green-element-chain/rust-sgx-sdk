use crate::beans::student::Student;

use sqlite3::{access, SqliteError};
use sqlite3::{
    Access, DatabaseConnection, QueryFold, ResultRowAccess, SqliteResult, StatementUpdate,
};
use sqlitedb::sqlite;
use sqlitedb::sqlops::lose;

pub fn base_student_ops(conn: &mut DatabaseConnection, &exist_flag: &bool) {
    println!("----------------student base operation ------------------");
    //setp 1 : create student table; insert some data;
    if !&exist_flag {
        println!("----------------------------------");
        create_student_table(conn);
        println!("----------------------------------");

        //step 2: insert bench data;
        println!("----------------------------------");
        insert_bench_student(conn);
        println!("----------------------------------");

        //step 3: delete student
        println!("----------------------------------");
        delete_student(conn);
        println!("----------------------------------");

        //step 4 : select student sum
        println!("----------------------------------");
        select_student_sum(conn);
        println!("----------------------------------");

        //step 5 : search student list
        println!("----------------------------------");
        match select_student_list(conn) {
            Ok(y) => {
                println!("SELECT * FROM student");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref()),
        }
    }

    println!("----------------database operations end------------------");
}

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
    for (_i, j) in (0..5).enumerate() {
        println!("the {} data", j);
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

pub fn insert_student(conn: &mut DatabaseConnection, student: &mut Student) {
    let mut tx;
    let mut txs;
    loop {
        tx = conn.prepare(
            "INSERT INTO student (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
                           VALUES ($1, $2, $3,$4, $5, $6,$7, $8,$9)",
        );
        match tx {
            Ok(T) => {
                txs = T;
            }
            Err(e) => {
                match sqlite::start_db(1) {
                    Ok(x) => {
                        *conn = x;
                        println!("reset conn");
                        continue;
                    }
                    _ => panic!("create database failed"),
                }
                println!("we get a error in prepare,retry again!");
            }
        }

        let mut change;
        let mut changes;

        changes = txs.update(&[
            &student.id,
            &student.street,
            &student.city,
            &student.sendstatus,
            &student.datatype,
            &student.ops,
            &student.age,
            &student.clientid,
            &student.indexid,
        ]);

        match changes {
            Ok(T) => {
                change = T;
                assert_eq!(change, 1);
                println!("insert student success");
                break;
            }
            Err(e) => {
                println!("{}", e);
                println!("insert student failed");
                break;
            }
        }
    }
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

pub fn delete_student(conn: &mut DatabaseConnection) {
    println!("delete data FROM student");
    let mut stmt2 = conn.prepare("DELETE FROM student WHERE ID = 4").unwrap();
    let mut results = stmt2.execute();
    match results.step() {
        Ok(Some(ref mut row1)) => {
            println!("delete success");
        }
        Err(oops) => panic!(oops),
        Ok(None) => println!("delete success"),
    }
}

pub fn select_student_list(conn: &mut DatabaseConnection) -> SqliteResult<Vec<Student>> {
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
