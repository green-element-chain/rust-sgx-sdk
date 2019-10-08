//! 分账交易的处理模块，账单分账记录管理等
//use client::HttpClient;
use config::{ApplicationConfig, UnionpayTranParam, UnionpayTranUrl};
use service::project::{
    dto::*,
    project_bill::ProjectBillMgr,
    project_bill_payment::ProjectBillPaymentMgr,
    project_receipt::ProjectReceiptMgr,
};
use service::response::SgxServerResponse;
use service::transaction::dto::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::{String, ToString};
use std::sync::Arc;
use utils::channel::unionpay::{self, sdkconstants as cst};
use utils::db::DbContext;
use utils::time;

mod dto;

#[derive(Clone)]
pub struct TransactionMgr {
    config: Rc<ApplicationConfig>,
    resp_code: ResponseCode,
    //http_client: HttpClient,
    receipt_mgr: ProjectReceiptMgr,
    bill_mgr: ProjectBillMgr,
    bill_payment_mgr: ProjectBillPaymentMgr,
}

#[allow(unused_variables)]
impl TransactionMgr {
    pub fn new(app_config: &Rc<ApplicationConfig>, context: &Arc<DbContext>) -> TransactionMgr {
        let transaction_mgr = TransactionMgr {
            config: app_config.clone(),
            resp_code: ResponseCode::new(),
            //http_client: HttpClient::new(),
            receipt_mgr: ProjectReceiptMgr::new(context),
            bill_mgr: ProjectBillMgr::new(context),
            bill_payment_mgr: ProjectBillPaymentMgr::new(context),
        };
        transaction_mgr
    }

    fn get_payment_request(&self, data: &TransRequestParam, tran_with_b2b: bool) -> Option<String> {
        let union_url: &UnionpayTranUrl = self.config.unionpay_tran_url();
        let union_param: &UnionpayTranParam = self.config.unionpay_tran_param();
        let split_amount = data.amount.to_string();
        let (tran_date, tran_time) = time::now_date_time();

        let mut req_data = HashMap::new();
        req_data.insert(cst::PARAM_VERSION, cst::FAST_PAY_VERSION);
        req_data.insert(cst::PARAM_MER_ID, union_param.mer_id().as_str());
        req_data.insert(cst::PARAM_ORDER_ID, data.order_no.as_str());
        req_data.insert(cst::PARAM_TXN_DATE, tran_date.as_str());
        req_data.insert(cst::PARAM_TXN_TIME, tran_time.as_str());
        req_data.insert(cst::PARAM_TXN_AMT, split_amount.as_str());
        req_data.insert(cst::PARAM_BIZ_TYPE, cst::BUSI_TYPE);
        req_data.insert(cst::PARAM_BACK_URL, union_param.notify_tran_url().as_str());
        req_data.insert(cst::PARAM_REMOTE_ADDR, union_param.remote_ip().as_str());

        //分账信息
        let split_method = format!("{}", data.split_method);
        let split_message = data.split_msg.replace(cst::PERCENT_SIGN, cst::BLANK);
        req_data.insert(cst::PARAM_SPLIT_TYPE, cst::SPLIT_TYPE);
        req_data.insert(cst::PARAM_SPLIT_METHOD, split_method.as_str());
        req_data.insert(cst::PARAM_MER_SPLIT_MSG, split_message.as_str());

        let mut customer_info: String;
        if !tran_with_b2b {
            //非B2B交易，需要填写分账需要的交易卡域信息
            let opt_receipt = self.receipt_mgr.get_project_receipt(data.project_id as u32);
            if opt_receipt.is_none() {
                error!("can't get receipt for project {}", data.project_id);
                return None;
            }
            let receipt = opt_receipt.unwrap();
            let cert_type = format!("{:02}", receipt.certType);

            let mut customer = HashMap::new();
            customer.insert(cst::PARAM_ACC_NAME, receipt.cardUser.as_str());
            customer.insert(cst::PARAM_CARD_NO, receipt.cardNum.as_str());
            customer.insert(cst::PARAM_CERT_TYPE, cert_type.as_str());
            customer.insert(cst::PARAM_CERT_NO, receipt.certNo.as_str());

            customer_info = unionpay::convert_to_json_str(&customer);
            customer_info = base64::encode(&customer_info);
            req_data.insert(cst::PARAM_CUSTOMER_INFO, customer_info.as_str());
            req_data.insert(cst::PARAM_TXN_TYPE, cst::TRAN_TYPE);
            req_data.insert(cst::PARAM_POST_URL, union_url.tran_url().as_str());
        } else {
            //B2B交易，增加前端交易的URL地址
            req_data.insert(cst::PARAM_POST_URL, union_url.tran_url_b2b().as_str());
            req_data.insert(cst::PARAM_FRONT_URL, union_param.notify_tran_url_b2b().as_str());
            req_data.insert(cst::PARAM_TXN_TYPE, union_param.tran_type_b2b().as_str());
        }

        Some(unionpay::convert_to_url_param_str(&req_data))
    }

    fn get_query_request(&self, data: &QueryRequestParam) -> String {
        let union_url: &UnionpayTranUrl = self.config.unionpay_tran_url();
        let union_param: &UnionpayTranParam = self.config.unionpay_tran_param();
        let trans_dt = time::get_time(data.tran_time);
        let trans_date: String = time::format_date(&trans_dt);

        let mut req_data = HashMap::new();
        req_data.insert(cst::PARAM_VERSION, cst::SIGN_VERSION);
        req_data.insert(cst::PARAM_MER_ID, union_param.mer_id().as_str());
        req_data.insert(cst::PARAM_ORDER_ID, data.order_no.as_str());
        req_data.insert(cst::PARAM_TXN_DATE, trans_date.as_str());
        req_data.insert(cst::PARAM_TXN_TYPE, cst::TRAN_TYPE_QUERY);
        req_data.insert(cst::PARAM_BIZ_TYPE, cst::BUSI_TYPE);
        req_data.insert(cst::PARAM_POST_URL, union_url.query_url().as_str());

        unionpay::convert_to_url_param_str(&req_data)
    }
}

// 所有的Restful接口实现
#[allow(unused_variables)]
impl TransactionMgr {
    pub fn restful_trans_params(&self, param: String) -> String {
        let payment_bill: PaymentBill = serde_json::from_str(param.as_str()).expect("Can't deserialize");

        let message = format!("can't find ledger or status invalid with bill {}", payment_bill.bill);
        let opt_tran_param: Option<TransRequestParam> = self.bill_mgr.get_transaction_params(&payment_bill);
        match opt_tran_param {
            None => { SgxServerResponse::failed(message) }
            Some(ref tran_param) => {
                let opt_request = self.get_payment_request(tran_param, payment_bill.b2b);
                match opt_request {
                    None => { SgxServerResponse::failed(message) }
                    Some(data) => {
                        let payment_method = if payment_bill.b2b { PaymentMethod::MannualB2B } else { PaymentMethod::SystemAuto };
                        let (payment_tran_time, result) = self.bill_payment_mgr.create(&data, payment_method);
                        if !result {
                            return SgxServerResponse::failed(
                                format!("failed to create payment record for bill {}", payment_bill.bill));
                        }

                        let real_tran_time = time::get_time(payment_tran_time);
                        info!("initiate b2b transaction bill[{}], tran_time[{}]", payment_bill.bill, real_tran_time);
                        let bill_update = BillUpdate::new(
                            &tran_param.order_no,
                            from_bill_status(BillStatus::Processing),
                            payment_tran_time,
                            time::now().timestamp());
                        self.bill_mgr.update_bill(&bill_update);

                        SgxServerResponse::success(data)
                    }
                }
            }
        }
    }

    pub fn restful_query_params(&self, param: String) -> String {
        let payment_bill: PaymentBill = serde_json::from_str(param.as_str()).expect("Can't deserialize");

        let message = format!("can't find bill {}", payment_bill.bill);
        let opt_query_param: Option<QueryRequestParam> = self.bill_mgr.get_query_params(&payment_bill);
        match opt_query_param {
            None => { SgxServerResponse::failed(message) }
            Some(ref query_param) => {
                let data = self.get_query_request(query_param);
                SgxServerResponse::success(data)
            }
        }
    }

    pub fn restful_payment_record_update(&self, param: String) -> String {
        let msg = "payment_record_update data to sgx server";
        info!("record_update data: {}", param);
        let mut map: HashMap<&str, &str> = unionpay::convert_from_json_str(&param);
        let order_no = map.get(cst::PARAM_ORDER_ID).unwrap().parse::<String>().unwrap();
        if order_no.is_empty() {
            return SgxServerResponse::failed(format!("{}", "can't find order no."));
        }

        let opt_update_info = self.bill_payment_mgr.get_update_record(&order_no);
        if opt_update_info.is_some() {
            let mut update_info = opt_update_info.unwrap();
            let payment_tran_time = time::get_time(update_info.tran_time);
            info!("transaction update, order_no[{}], tran_time[{}]", order_no, payment_tran_time);
            let (_, status_result) = self.update_payment_status(&mut update_info, &mut map, cst::PARAM_RESP_CODE);
            if status_result {
                return SgxServerResponse::success(format!("{}", msg));
            }
        }

        SgxServerResponse::success(String::from(msg))
    }

    pub fn restful_payment_record_get(&self, param: String) -> String {
        let result = self.bill_payment_mgr.get_all();
        if result.is_none() {
            return SgxServerResponse::failed(format!("{}", "can't find payment record."));
        }
        SgxServerResponse::success(result.unwrap())
    }

    pub fn restful_notify(&self, param: String) -> String {
        debug!("notify info: {}", param);
        let mut map = unionpay::convert_from_json_str(&param);
        let order_id = map.get(cst::PARAM_ORDER_ID).unwrap().parse::<String>().unwrap();
        if order_id.is_empty() {
            return SgxServerResponse::failed(format!("{}", "can't find order no."));
        }

        let opt_update_info = self.bill_payment_mgr.get_update_record(&order_id);
        if opt_update_info.is_some() {
            let mut update_info = opt_update_info.unwrap();
            let payment_tran_time = time::get_time(update_info.tran_time);
            info!("receive transaction notify, order_no[{}], tran_time[{}]", order_id, payment_tran_time);

            let (payment_status, status_result) =
                self.update_payment_status(&mut update_info, &mut map, cst::PARAM_ORDER_STATUS);
            if status_result {
                let mut redirect_url = String::new();
                let notice = map.get(cst::PARAM_NOTICE);
                if notice.is_some() && notice.unwrap().parse::<bool>().unwrap() {
                    let value = match payment_status {
                        PaymentStatus::Failed => { 1 }
                        PaymentStatus::Success => { 0 }
                        _ => { -1 }
                    };
                    redirect_url = format!("{}?tab={}", self.config.unionpay_tran_param().redirect_url().as_str(), value);
                }
                return SgxServerResponse::success_ext(redirect_url, String::from("ok"));
            }
        }

        SgxServerResponse::failed(format!("{}", "notify can't find payment record."))
    }

    fn update_payment_status(&self, update_info: &mut PaymentUpdate, map: &mut HashMap<&str, &str>, status_key: &str) -> (PaymentStatus, bool) {
        update_info.resp_code = unionpay::get_num_from_map(&map, cst::PARAM_RESP_CODE);
        update_info.resp_msg = unionpay::get_str_from_map(&map, cst::PARAM_RESP_MSG);
        update_info.acq_seq_id = unionpay::get_str_from_map(&map, cst::PARAM_ACQ_SEQ_ID);
        update_info.update_time = time::now().timestamp();

        let order_status = map.get(status_key).unwrap().parse::<i16>().unwrap();
        let (payment_status, status_value) = self.resp_code.get_payment_status(order_status);
        update_info.status = status_value;
        if self.bill_payment_mgr.update_status(&update_info) {
            let bill_update = BillUpdate::convert(&update_info);
            return (payment_status, self.bill_mgr.update_bill(&bill_update));
        }
        (payment_status, false)
    }
}