//! 分账交易的处理模块，账单分账记录管理等
use client::HttpClient;
use config::{ApplicationConfig, UnionpayTranParam, UnionpayTranUrl};
use service::project::{
    dto::*,
    project_bill::ProjectBillMgr,
    project_bill_payment::ProjectBillPaymentMgr,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::channel::unionpay::{sdkconstants as cst, sdkutils};
use utils::db::DbContext;
use utils::time;

mod dto;

#[derive(Clone)]
pub struct TransactionMgr {
    app_config: Rc<ApplicationConfig>,
    http_client: HttpClient,
    bill_mgr: ProjectBillMgr,
    bill_payment_mgr: ProjectBillPaymentMgr,
}

#[allow(unused_variables)]
impl TransactionMgr {
    pub fn new(config: &Rc<ApplicationConfig>, context: &Arc<DbContext>) -> TransactionMgr {
        let transaction_mgr = TransactionMgr {
            app_config: config.clone(),
            http_client: HttpClient::new(),
            bill_mgr: ProjectBillMgr::new(context),
            bill_payment_mgr: ProjectBillPaymentMgr::new(context),
        };
        transaction_mgr
    }

    fn get_payment_request(&self, data: &TranRequestParam, tran_with_b2b: bool) -> String {
        let union_url: &UnionpayTranUrl = self.app_config.unionpay_tran_url();
        let union_param: &UnionpayTranParam = self.app_config.unionpay_tran_param();
        let (tran_date, tran_time) = time::now_date_time();

        /*let mut reqData = HashMap::new();
        reqData.insert(cst::PARAM_VERSION, sdkutils::FAST_PAY_VERSION);
        reqData.insert(cst::PARAM_MER_ID, union_param.mer_id());
        reqData.insert(cst::PARAM_ORDER_ID, data.order_no);
        reqData.insert(cst::PARAM_TXN_DATE, tran_date);
        reqData.insert(cst::PARAM_TXN_TIME, tran_time);
        reqData.insert(cst::PARAM_TXN_AMT, String::from(data.amount).as_str());
        reqData.insert(cst::PARAM_BIZ_TYPE, sdkutils::BUSI_TYPE);
        reqData.insert(cst::PARAM_BACK_URL, union_url.tran_url());
        reqData.insert(cst::PARAM_REMOTE_ADDR, union_param.remote_ip());

        //分账信息
        reqData.insert(cst::PARAM_SPLIT_TYPE, sdkutils::SPLIT_TYPE);
        reqData.insert(cst::PARAM_SPLIT_METHOD, "");
        reqData.insert(cst::PARAM_MER_SPLIT_MSG, data.split_msg.replace(cst::PERCENT_SIGN, cst::BLANK));

        //非B2B交易，需要填写分账需要的交易卡域信息
        if !tran_with_b2b {
            let mut customerData = HashMap::new();

            let customerInfo = "";//ChinaPayUtils.getCustomerInfo(customerInfoMap, secUtil);
            reqData.insert(cst::PARAM_CUSTOMER_INFO, customerInfo);
            reqData.insert(cst::PARAM_TXN_TYPE, sdkutils::TRAN_TYPE);
        } else {
            reqData.insert(cst::PARAM_FRONT_URL, union_url.tran_url_b2b());
            reqData.insert(cst::PARAM_TXN_TYPE, union_param.tran_type_b2b());
        }*/

        //对数据签名

        //将参数Map转换转换为字符串

        /*contentData.put(sdkconstants.param_version, ChinaPayUtils.FastPayVersion);
        contentData.put(sdkconstants.param_merId, properties.getMerId());
        contentData.put(sdkconstants.param_orderId, bill.getOrderNo());
        contentData.put(sdkconstants.param_txnDate, transDT.getTranDate());
        contentData.put(sdkconstants.param_txnTime, transDT.getTranTime());
        contentData.put(sdkconstants.param_txnAmt, bill.getAmount());
        contentData.put(sdkconstants.param_bizType, ChinaPayUtils.BUSI_TYPE);
        contentData.put(sdkconstants.param_backUrl, properties.getBackUrl());
        contentData.put(sdkconstants.param_remoteAddr, properties.getRemoteAddr());

        contentData.put(sdkconstants.param_splitType, ChinaPayUtils.SPLIT_TYPE);
        contentData.put(sdkconstants.param_splitMethod, bill.getSplitMethod().toString());
        contentData.put(sdkconstants.param_merSplitMsg,
            bill.getSplitMessage().replaceAll(sdkconstants.PERCENT_SIGN, sdkconstants.BLANK));*/

        //交易卡信息域
        /*if (!userB2BPayment) {
            ProjectReceipt receipt = bill.getReceipt();
            Map<String, String> customerInfoMap = new HashMap<>(4);
            customerInfoMap.put(SDKConstants.param_accName, receipt.getName());
            customerInfoMap.put(SDKConstants.param_cardNo, receipt.getCardNum());
            customerInfoMap.put(SDKConstants.param_certType, String.format("%02d", receipt.getCredentialTypeId()));
            customerInfoMap.put(SDKConstants.param_certNo, receipt.getIdNum());
            String customerInfo = ChinaPayUtils.getCustomerInfo(customerInfoMap, secUtil);
            contentData.put(SDKConstants.param_customerInfo, customerInfo);
            contentData.put(SDKConstants.param_txnType, ChinaPayUtils.TRAN_TYPE);
        } else {
            contentData.put(SDKConstants.param_frontUrl, properties.getFrontUrl());
            contentData.put(SDKConstants.param_txnType, properties.getFrontType());
        }*/

        String::from("")
    }
}

// 所有的Restful接口实现
#[allow(unused_variables)]
impl TransactionMgr {
    pub fn restful_payment(&self, param: String) -> String {
        let payment_param: PaymentParam = serde_json::from_str(param.as_str()).expect("Can't deserialize");
        let tran_params: Vec<TranRequestParam> = self.bill_mgr.get_tran_params(payment_param);

        let post_url: &String = self.app_config.unionpay_tran_url().tran_url();
        for data in tran_params.iter() {
            info!("payment bill {:?}", data);
            //let param = String::from("");
            /*let result = self.http_client.send_post(post_url, param.as_str());
            info!("test result: {}", result);*/
        }

        String::from("payment from server")
    }

    pub fn payment_b2b(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
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