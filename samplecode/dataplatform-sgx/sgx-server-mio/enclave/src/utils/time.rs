use chrono::{NaiveDate, NaiveDateTime};
use chrono::Local;

use std::ops::Add;
use std::string::{String, ToString};

pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn now_str() -> String {
    let local_time = now();
    format(&local_time)
}

//时间格式："2019-08-30 10:20:50"
pub fn format(time: &NaiveDateTime) -> String {
    time.format("%F %H:%M:%S").to_string()
}

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

pub fn parse_native_time_from_seconds(secs: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(secs, 0)
}

//分账的日期和时间，格式："20190902", "182050"
pub fn now_date_time() -> (String, String) {
    let local_time = now();
    (local_time.format("%Y-%m-%d").to_string(), local_time.format("%H%M%S").to_string())
}

//获取账单的订单编号，发起交易使用
pub fn get_order_no(time: &NaiveDateTime) -> String {
    let order_no = time.format("%Y%m%d%H%M%S%3f").to_string();
    //let number = rand::thread_rng().gen_range(65, 90);
    order_no
}