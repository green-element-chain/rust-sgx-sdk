use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

use std::string::String;
use std::vec::Vec;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectAsset {
    pub projectId: i64,
    pub assets: Vec<i64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectLedger {
    pub projectId: u32,
    pub billDate: u8,
    pub billStartDate: String,
    pub billCycle: u8,
    pub ledgerDate: u16,
    pub ledgerContent: String,
}

impl ProjectLedger {
    pub fn new() -> ProjectLedger {
        ProjectLedger {
            projectId: 0,
            billDate: 0,
            billStartDate: String::from(""),
            billCycle: 1,
            ledgerDate: 0,
            ledgerContent: String::from(""),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BillStatus {
    //-1 支付失败
    Failed,
    // 0 支付成功
    Success,
    // 1 待支付
    Initialize,
    // 2 已交易，处理中
    Processing,
}

pub fn from_bill_status(status: BillStatus) -> i16 {
    let ret_value = match status {
        BillStatus::Failed => -1,
        BillStatus::Success => 0,
        BillStatus::Initialize => 1,
        BillStatus::Processing => 2,
    };
    ret_value as i16
}

pub fn to_bill_status(value: i16) -> BillStatus {
    let ret_value = match value {
        -1 => BillStatus::Failed,
        0 => BillStatus::Success,
        1 => BillStatus::Initialize,
        2 => BillStatus::Processing,
        _ => {
            let message = format!("invalid bill status value {}.", value);
            panic!(message);
        }
    };
    ret_value
}

#[derive(Debug)]
pub struct ProjectBillTimeRange {
    pub bill_start_time: Option<NaiveDateTime>,
    pub bill_end_time: Option<NaiveDateTime>,
}

impl ProjectBillTimeRange {
    pub fn new(start_time: Option<NaiveDateTime>, end_time: Option<NaiveDateTime>) -> ProjectBillTimeRange {
        ProjectBillTimeRange {
            bill_start_time: start_time,
            bill_end_time: end_time,
        }
    }
}
