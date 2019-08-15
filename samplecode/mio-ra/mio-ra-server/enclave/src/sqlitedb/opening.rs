use std::default::Default;
use std::io::Write;
use std::prelude::v1::*;
use std::vec::Vec;
use sgx_types::*;

use sqlite3::{
    Access,
    DatabaseConnection,
    QueryFold,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};
use sqlite3::access;
use sqlite3::access::flags::Flags;
use std::untrusted::fs::File;

use crate::beans::teacher::Teacher;

pub fn opening(existed:uint8_t) {
    let args : Vec<String> = Vec::new();
    let usage = "sqlite";


    let cli_access = {
        let ok = |flags, dbfile| Some(access::ByFilename { flags: flags, filename: dbfile });

        let arg = |n| {
            if args.len() > n { Some(args[n].as_ref()) }
            else { None }
        };

        match (arg(1), arg(2)) {
            (Some("-r"), Some(dbfile))
            => ok(Flags::OPEN_READONLY, dbfile),
            (Some(dbfile), None)
            => ok(Default::default(), dbfile),
            (_, _) => {
                let dbfile = "test.db";
                ok(Default::default(), dbfile)
            }
        }
    };

    println!("test_openings success!");

    fn use_access<A: Access>(access: A, existed: uint8_t) -> SqliteResult<Vec<Teacher>> {
        let mut conn = try!(DatabaseConnection::new(access));
        let mut exist_flag = false;
        let mut number = 1;
        if (existed == 1) {
            exist_flag = true
        }

        make_people(&mut conn, exist_flag)
    }


    fn lose(why: &str) {
        // FIXME: Set the exit status once that is stabilized
        let stderr = std::io::stderr();
        let mut stderr_lock = stderr.lock();
        stderr_lock.write_fmt(format_args!("{}", why)).unwrap()
    }

    match cli_access {
        Some(a) => match use_access(a,existed) {
            Ok(x) => println!("Ok: {:?}", x),
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref())
        },
        None => lose(usage)
    }
}


#[derive(Debug, Clone)]
struct Person {
    id: i32,
    name: String,
    price: i32,
}



fn use_access<A: Access>(access: A, existed: bool) -> SqliteResult<Vec<Teacher>> {
    let mut conn = DatabaseConnection::new(access)?;
    make_people(&mut conn, existed)
}

fn lose(why: &str) {
    // FIXME: Set the exit status once that is stabilized
    let stderr = std::io::stderr();
    let mut stderr_lock = stderr.lock();
    stderr_lock.write_fmt(format_args!("{}", why)).unwrap()
}

fn make_people(conn: &mut DatabaseConnection, existed: bool) -> SqliteResult<Vec<Teacher>> {
    if !existed {
        println!("table not existed!");

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
        )?;

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

            let mut tx = conn.prepare(
                "INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
                           VALUES ($1, $2, $3,$4, $5, $6,$7, $8,$9)",
            )?;
            let changes = tx.update(&[&teacher.id, &teacher.street, &teacher.city,
                &teacher.sendstatus,&teacher.datatype,&teacher.ops,&teacher.age,
                &teacher.clientid,&teacher.indexid])?;
            assert_eq!(changes, 1);
        }
    }else{
        println!("db existed!table existed!");
    }

    //select teacher
    let mut stmt2 = conn.prepare("SELECT sum(clientid) FROM teacher")?;
    let mut results = stmt2.execute();
    match results.step() {
        Ok(Some(ref mut row1)) => {
            let id = row1.column_int(0);

            println!("row: {}",id);
        }
        Err(oops) => panic!(oops),
        Ok(None) => panic!("where did our row go?"),
    }
    println!("SELECT sum(clientid) FROM teacher");

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
                indexid:row.get(8),
            },
            ppl,
        ))
    })?;
    Ok(ppl)


}
