use crate::beans::student::Student;
use crate::beans::teacher::Teacher;
use crate::sqlitedb;

pub fn buzfn(inputstr: &str) -> &str {
    println!("this is business logic in the app");
    println!("the input str is {} in the app", inputstr);

    //default is teacher, datatype = 0
    //student, datatype = 1
    let mut datatype = 2;

    match inputstr.find("energys_teacher") {
        Some(T) => {
            datatype = 0;
            println!("datatype is teacher!");
        }
        _ => {
            println!("datatype isn't student!");
        }
    }
    match inputstr.find("energy_student") {
        Some(T) => {
            datatype = 1;
            println!("datatype is student!");
        }
        _ => println!("datatype isn't teacher!"),
    }

    //startdb
    let mut conn;
    match sqlitedb::sqlite::start_db(0) {
        Ok(x) => conn = x,
        _ => panic!("create database failed"),
    }

    if datatype == 1 {
        println!("datatype is 1");
        let result: Student = serde_json::from_str(inputstr).unwrap();
    } else if datatype == 0 {
        println!("datatype is 0");
        let result: Teacher = serde_json::from_str(inputstr).unwrap();
        let mut teacher = result.clone();

        if result.sendstatus == "end" {
            sqlitedb::teacherdao::insert_teacher(&mut conn, &mut teacher);
            let resultlist = sqlitedb::teacherdao::select_teacher_list(&mut conn).unwrap();
            println!("{:?}",resultlist);
            println!("insert data success");
        } else {
            sqlitedb::teacherdao::insert_teacher(&mut conn, &mut teacher);
            println!("start to insert data");
            let resultlist = sqlitedb::teacherdao::select_teacher_list(&mut conn).unwrap();
            println!("{:?}",resultlist);
            println!("insert data success");
        }
    } else {
        println!("datatype is 2");
    }

    "\"this is  a output str\""
}
