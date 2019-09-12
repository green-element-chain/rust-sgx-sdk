//! 分账交易的处理模块，账单分账记录管理等
use client::HttpClient;
use config::{ApplicationConfig, UnionpayTranParam, UnionpayTranUrl};
use service::project::{
    dto::*,
    project_bill::ProjectBillMgr,
    project_bill_payment::ProjectBillPaymentMgr,
    project_receipt::ProjectReceiptMgr,
};
use service::response::SgxServerResponse;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;
use utils::channel::unionpay::{self, sdkconstants as cst, sdkutil::SDKUtil};
use utils::db::DbContext;
use utils::time;

mod dto;

#[derive(Clone)]
pub struct TransactionMgr {
    config: Rc<ApplicationConfig>,
    sdk_utils: SDKUtil,
    http_client: HttpClient,
    receipt_mgr: ProjectReceiptMgr,
    bill_mgr: ProjectBillMgr,
    bill_payment_mgr: ProjectBillPaymentMgr,
}

#[allow(unused_variables)]
impl TransactionMgr {
    pub fn new(app_config: &Rc<ApplicationConfig>, context: &Arc<DbContext>) -> TransactionMgr {
        let transaction_mgr = TransactionMgr {
            config: app_config.clone(),
            sdk_utils: SDKUtil::new(app_config.unionpay_tran_param()),
            http_client: HttpClient::new(),
            receipt_mgr: ProjectReceiptMgr::new(context),
            bill_mgr: ProjectBillMgr::new(context),
            bill_payment_mgr: ProjectBillPaymentMgr::new(context),
        };
        transaction_mgr
    }

    fn get_payment_request(&self, data: &TranRequestParam, tran_with_b2b: bool) -> Option<String> {
        let union_url: &UnionpayTranUrl = self.config.unionpay_tran_url();
        let union_param: &UnionpayTranParam = self.config.unionpay_tran_param();
        let split_amount = data.amount.to_string();
        let (tran_date, tran_time) = time::now_date_time();

        let mut req_data = BTreeMap::new();
        /*req_data.insert(cst::PARAM_VERSION, sdkutil::FAST_PAY_VERSION);
        req_data.insert(cst::PARAM_MER_ID, union_param.mer_id());
        req_data.insert(cst::PARAM_ORDER_ID, data.order_no.clone());
        req_data.insert(cst::PARAM_TXN_DATE, tran_date);
        req_data.insert(cst::PARAM_TXN_TIME, tran_time);
        req_data.insert(cst::PARAM_TXN_AMT, split_amount);
        req_data.insert(cst::PARAM_BIZ_TYPE, sdkutil::BUSI_TYPE);
        req_data.insert(cst::PARAM_BACK_URL, union_url.tran_url());
        req_data.insert(cst::PARAM_REMOTE_ADDR, union_param.remote_ip());

        //分账信息
        let split_method = format!("{}", data.split_method);
        let split_message = data.split_msg.replace(cst::PERCENT_SIGN, cst::BLANK);
        req_data.insert(cst::PARAM_SPLIT_TYPE, sdkutil::SPLIT_TYPE);
        req_data.insert(cst::PARAM_SPLIT_METHOD, split_method);
        req_data.insert(cst::PARAM_MER_SPLIT_MSG, split_message);

        //非B2B交易，需要填写分账需要的交易卡域信息
        if !tran_with_b2b {
            let opt_receipt: Option<ProjectReceipt> = self.receipt_mgr.get_project_receipt_one(data.project_id as u32);
            if opt_receipt.is_none() {
                error!("can't get receipt for project {}", data.project_id);
                return None;
            }

            let mut customer = BTreeMap::new();
            let receipt = opt_receipt.unwrap();
            let cert_type = format!("{:02}, ", receipt.certType);
            customer.insert(cst::PARAM_CERT_TYPE, cert_type);
            customer.insert(cst::PARAM_CERT_NO, receipt.certNo);
            customer.insert(cst::PARAM_ACC_NAME, receipt.cardUser);
            customer.insert(cst::PARAM_CARD_NO, receipt.cardNum);

            let customer_info = self.sdk_utils.get_customer_info(&customer);
            req_data.insert(cst::PARAM_CUSTOMER_INFO, customer_info);
            req_data.insert(cst::PARAM_TXN_TYPE, sdkutil::TRAN_TYPE);
        } else {
            req_data.insert(cst::PARAM_FRONT_URL, union_url.tran_url_b2b());
            req_data.insert(cst::PARAM_TXN_TYPE, union_param.tran_type_b2b());
        }*/

        //对数据签名
        if self.sdk_utils.sign(&mut req_data) {
            Some(unionpay::get_signed_param_str(&req_data))
        } else {
            error!("failed to sign transaction data for project {}", data.project_id);
            None
        }
    }
}

// 所有的Restful接口实现
#[allow(unused_variables)]
impl TransactionMgr {
    pub fn restful_payment(&self, param: String) -> String {
        let msg = "restful_payment transaction from sgx server";
        let payment_param: PaymentParam = serde_json::from_str(param.as_str()).expect("Can't deserialize");
        let tran_params: Vec<TranRequestParam> = self.bill_mgr.get_tran_params(payment_param);

        let mut succ_count: u32 = 0;
        let mut fail_count: u32 = 0;
        let post_url: String = self.config.unionpay_tran_url().tran_url();
        for data in tran_params.iter() {
            info!("payment bill {:?}", data);
            let opt_param = self.get_payment_request(data, false);
            if opt_param.is_none() {
                fail_count += 1;
                error!("failed to get payment request for project {}", data.project_id);
                continue;
            }
            let param = opt_param.unwrap();
            debug!("url: {}", post_url);
            debug!("param: {}", param.as_str());
            let result = self.http_client.send_post(post_url.as_str(), param.as_str());
            info!("test result: {}", result);

            succ_count += 1;
        }

        String::from("payment from server");
        SgxServerResponse::success(format!("{} succeed({}), failed({})", msg, succ_count, fail_count))
    }

    pub fn payment_b2b(&self, param: String) -> String {
        //let mut req_data = BTreeMap::new();
        //self.sdk_utils.sign(&mut req_data);

        //let post_url = String::from("https://tieba.baidu.com/");
        let post_url: String = self.config.unionpay_tran_url().tran_url();
        info!("request url: {}", post_url.as_str());
        self.http_client.send_post(post_url.as_str(), "");

        String::from("payment_b2b from server")
    }

    pub fn notify(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        String::from("notify from server")
    }

    pub fn notify_b2b(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        String::from("notify_b2b from server")
    }
}