use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

use std::string::String;
use std::vec::Vec;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectBillReq {
    pub day: u16,
    pub projects: Vec<u32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectAssetReq {
    pub projectId: u32,
    pub assets: Vec<u32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectLedger {
    pub projectId: i32,
    pub billDate: i32,
    pub billStartDate: String,
    pub billCycle: i32,
    pub ledgerDate: i32,
    pub ledgerMode: i32,
    pub ledgerContent: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectReceipt {
    pub projectId: i32,
    pub chargeMode: i32,
    pub cardNum: String,
    pub cardUser: String,
    pub certType: i32,
    pub certNo: String,
    pub mobile: String,
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectBill {
    pub id: i32,
    pub projectId: i32,
    pub beginDate: String,
    pub endDate: String,
    pub amount: i64,
    pub orderNo: String,
    pub status: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentParam {
    pub bill: u32,
    pub day: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranRequestParam {
    pub project_id: i32,
    pub split_msg: String,
    pub split_method: i32,
    pub order_no: String,
    pub amount: i64,
}
