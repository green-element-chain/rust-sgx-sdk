use chrono::{NaiveDate, NaiveDateTime, Timelike};
use chrono::Local;

use std::ops::Add;
use std::string::{String, ToString};

pub fn parse_native_date_from_str(date: String) -> NaiveDate {
    let date_time = date.add(" 00:00:00");
    let resp_native_date = NaiveDateTime::parse_from_str(date_time.as_str(), "%F %H:%M:%S")
        .expect("Can't parse invalid date.");
    resp_native_date.date()
}

pub fn parse_native_time_from_str(date: String) -> NaiveDateTime {
    let date_time = if date.len() < 11 { date.add(" 00:00:00") } else { date };
    let resp_native_date = NaiveDateTime::parse_from_str(date_time.as_str(), "%F %H:%M:%S")
        .expect("Can't parse invalid date.");
    resp_native_date
}

pub fn parse_native_time_from_nanosecond(nano: u32) -> NaiveDateTime {
    let now = Local::now().naive_local();
    now.with_nanosecond(nano).unwrap()
}

pub fn get_order_no(time: NaiveDateTime) -> String {
    let order_no = time.format("%Y%m%d%H%M%S%3f").to_string();
    //let number = rand::thread_rng().gen_range(65, 90);
    order_no
}