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
            panic!(format!("invalid bill status value {}.", value));
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
    pub status: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BillUpdate {
    pub order_no: String,
    pub serial_no: Option<String>,
    pub status: i16,
    pub tran_time: i64,
    pub update_time: i64,
}

impl BillUpdate {
    pub fn new(order_id: &String, _status: i16, _tran_time: i64, _time: i64) -> BillUpdate {
        BillUpdate {
            order_no: order_id.clone(),
            serial_no: None,
            status: _status,
            tran_time: _tran_time,
            update_time: _time,
        }
    }

    pub fn convert(pu: &PaymentUpdate) -> BillUpdate {
        let bill_status = match to_payment_status(pu.status) {
            PaymentStatus::Failed => from_bill_status(BillStatus::Failed),
            PaymentStatus::Success => from_bill_status(BillStatus::Success),
            PaymentStatus::Processing => from_bill_status(BillStatus::Processing),
        };

        BillUpdate {
            order_no: pu.order_no.clone(),
            serial_no: pu.acq_seq_id.clone(),
            status: bill_status,
            tran_time: pu.tran_time,
            update_time: pu.update_time.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentBill {
    pub bill: u32,
    pub b2b: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransRequestParam {
    pub project_id: i32,
    pub split_msg: String,
    pub split_method: i32,
    pub order_no: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRequestParam {
    pub order_no: String,
    pub tran_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentRecord {
    pub id: i32,
    pub order_no: String,
    pub amount: i64,
    pub method: i16,
    pub tran_time: i64,
    pub status: i16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PaymentStatus {
    //-1 支付失败
    Failed,
    // 0 支付成功
    Success,
    // 1 处理中
    Processing,
}

pub fn from_payment_status(status: &PaymentStatus) -> i16 {
    let ret_value = match status {
        PaymentStatus::Failed => -1,
        PaymentStatus::Success => 0,
        PaymentStatus::Processing => 1,
    };
    ret_value as i16
}

pub fn to_payment_status(value: i16) -> PaymentStatus {
    let ret_value = match value {
        -1 => PaymentStatus::Failed,
        0 => PaymentStatus::Success,
        1 => PaymentStatus::Processing,
        _ => {
            panic!(format!("invalid payment status value {}.", value));
        }
    };
    ret_value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentUpdate {
    pub id: i32,
    pub order_no: String,
    pub tran_time: i64,
    pub status: i16,
    pub query_times: i32,
    pub update_time: i64,
    pub resp_code: Option<i32>,
    pub resp_msg: Option<String>,
    pub acq_seq_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PaymentMethod {
    //0 系统自动分账
    SystemAuto,
    //1 人工B2B支付
    MannualB2B,
}

pub fn from_payment_method(method: &PaymentMethod) -> i16 {
    let ret_value = match method {
        PaymentMethod::SystemAuto => 0,
        PaymentMethod::MannualB2B => 1,
    };
    ret_value as i16
}

pub fn to_payment_method(value: i16) -> PaymentMethod {
    let ret_value = match value {
        0 => PaymentMethod::SystemAuto,
        1 => PaymentMethod::MannualB2B,
        _ => {
            panic!(format!("invalid payment method value {}.", value));
        }
    };
    ret_value
}
