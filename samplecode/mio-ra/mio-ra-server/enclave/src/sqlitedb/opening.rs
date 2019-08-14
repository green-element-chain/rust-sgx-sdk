use std::default::Default;
use std::io::Write;
use std::prelude::v1::*;
use std::vec::Vec;
use std::untrusted::fs;

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

pub fn opening() {
    let args : Vec<String> = Vec::new();
    let usage = "sqlite";
    let mut filename = String::new();
    let mut existed;


    let cli_access = {
        let ok = |flags, dbfile| {
            Some(access::ByFilename {
                flags: flags,
                filename: dbfile,
            })
        };

        let arg = |n| {
            if args.len() > n {
                Some(args[n].as_ref())
            } else {
                None
            }
        };

        match (arg(1), arg(2)) {
            (Some(dbfile), None) => {
                filename = String::from("test.db");
                ok(Default::default(), dbfile)
            }
            (_, _) => {
                let dbfile = "test.db";
                filename = String::from("test.db");
                ok(Default::default(), dbfile)
            }
        }
    };

    let f = fs::File::open(filename);
    match f {
        Ok(_) => {
            existed = true;
            println!("db existed?: {}", existed)
        }
        Err(_) => {
            existed = false;
            println!("db existed?: {}", existed)
        }
    }

    match cli_access {
        Some(a) => match use_access(a, existed) {
            Ok(x) => println!("Ok: {:?}", x),
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref()),
        },
        None => lose(usage),
    }
}


#[derive(Debug, Clone)]
struct Person {
    id: i32,
    name: String,
    price: i32,
}
fn use_access<A: Access>(access: A, existed: bool) -> SqliteResult<Vec<Person>> {
    let mut conn = DatabaseConnection::new(access)?;
    make_people(&mut conn, existed)
}

fn lose(why: &str) {
    // FIXME: Set the exit status once that is stabilized
    let stderr = std::io::stderr();
    let mut stderr_lock = stderr.lock();
    stderr_lock.write_fmt(format_args!("{}", why)).unwrap()
}

fn make_people(conn: &mut DatabaseConnection, existed: bool) -> SqliteResult<Vec<Person>> {
    if !existed {
        conn.exec(
            "CREATE TABLE person (
                 id              SERIAL PRIMARY KEY,
                 name            VARCHAR NOT NULL,
                 price           integer
               )",
        )?;

        for (_i, j) in (0..50).enumerate() {
            let person = Person {
                id: j,
                name: "Dan".to_owned() + j.to_string().as_str(),
                price: j,
            };

            let mut tx = conn.prepare(
                "INSERT INTO person (id, name,price)
                           VALUES ($1, $2, $3)",
            )?;
            let changes = tx.update(&[&person.id, &person.name, &person.price])?;
            assert_eq!(changes, 1);
        }
    }

    let mut stmt2 = conn.prepare("SELECT sum(price) FROM person")?;
    let mut results = stmt2.execute();
    match results.step() {
        Ok(Some(ref mut row1)) => {
            let id = row1.column_int(0);

            println!("row: {}",id);
        }
        Err(oops) => panic!(oops),
        Ok(None) => panic!("where did our row go?"),
    }

    let mut stmt = conn.prepare("SELECT * FROM person")?;

    let snoc = |x, mut xs: Vec<_>| {
        xs.push(x);
        xs
    };

    let ppl = stmt.query_fold(&[], vec![], |row, ppl| {
        Ok(snoc(
            Person {
                id: row.get(0),
                name: row.get(1),
                price: row.get(2),
            },
            ppl,
        ))
    })?;
    Ok(ppl)

}
