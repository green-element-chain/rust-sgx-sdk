use sgx_types::*;
use std::default::Default;
use std::io::Write;
use std::prelude::v1::*;
use std::vec::Vec;

use crate::sqlitedb::sqlops;
use crate::sqlitedb::{studentdao, teacherdao};
use sqlite3::access;
use sqlite3::access::flags::Flags;
use sqlite3::{
    Access, DatabaseConnection, QueryFold, ResultRowAccess, SqliteResult, StatementUpdate,
};
use std::untrusted::fs::File;

use crate::beans::teacher::Teacher;

pub fn base_test(existed: uint8_t) {
    let args: Vec<String> = Vec::new();
    let usage = "sqlite";

    let mut conn;
    let mut exist_flag = false;
    let mut number = 1;
    if (existed == 1) {
        exist_flag = true
    }

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
            (Some("-r"), Some(dbfile)) => ok(Flags::OPEN_READONLY, dbfile),
            (Some(dbfile), None) => ok(Default::default(), dbfile),
            (_, _) => {
                let dbfile = "test.db";
                ok(Default::default(), dbfile)
            }
        }
    };

    match cli_access {
        Some(a) => match sqlops::get_database_conn(a) {
            Ok(x) => {
                conn = x;

                if !&exist_flag {
                    println!("----------------teacher base operation ------------------");
                    //setp 1 : create teacher table; insert some data;
                    println!("----------------------------------");
                    teacherdao::create_teacher_table(&mut conn);
                    println!("----------------------------------");

                    //step 2: insert bench data;
                    println!("----------------------------------");
                    teacherdao::insert_bench_teacher(&mut conn);
                    println!("----------------------------------");
                }

                //step 3 : select teacher sum
                println!("----------------------------------");
                teacherdao::select_teacher_sum(&mut conn);
                println!("----------------------------------");

                //step 4 : search teacher list
                println!("----------------------------------");
                match teacherdao::select_teacher_list(&mut conn, exist_flag) {
                    Ok(y) => {
                        println!("SELECT * FROM teacher");
                        println!("Ok: {:?}", y);
                    }
                    Err(oops) => lose(format!("oops!: {:?}", oops).as_ref()),
                }
                println!("----------------student base operation ------------------");
                //setp 1 : create teacher table; insert some data;
                if !&exist_flag {
                    println!("----------------------------------");
                    studentdao::create_student_table(&mut conn);
                    println!("----------------------------------");

                    //step 2: insert bench data;
                    println!("----------------------------------");
                    studentdao::insert_bench_student(&mut conn);
                    println!("----------------------------------");
                }

                //step 3 : select teacher sum
                println!("----------------------------------");
                studentdao::select_student_sum(&mut conn);
                println!("----------------------------------");

                //step 4 : search teacher list
                println!("----------------------------------");
                match studentdao::select_student_list(&mut conn, exist_flag) {
                    Ok(y) => {
                        println!("SELECT * FROM student");
                        println!("Ok: {:?}", y);
                    }
                    Err(oops) => lose(format!("oops!: {:?}", oops).as_ref()),
                }
                println!("----------------database operations end------------------");
            }
            Err(oops) => lose(format!("oops!: {:?}", oops).as_ref()),
        },
        None => lose(usage),
    }
}

fn lose(why: &str) {
    // FIXME: Set the exit status once that is stabilized
    let stderr = std::io::stderr();
    let mut stderr_lock = stderr.lock();
    stderr_lock.write_fmt(format_args!("{}", why)).unwrap()
}
