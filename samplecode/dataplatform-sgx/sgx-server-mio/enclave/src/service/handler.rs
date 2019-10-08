//! 该模块负责restful接口的业务分发，本模块不负责处理具体的业务。
//! 此封装方法存在的问题：
//! 1、第一次操作数据库会失败，提示"step [disk I/O error]"
//! 2、数据库自动生成的时间默认为UTC
use serde_derive::{Deserialize, Serialize};

use config::ApplicationConfig;
use service::order::OrderMgr;
use service::project::{project_utils::ProjectTable, ProjectMgr};
use service::transaction::TransactionMgr;
use std::rc::Rc;
use std::string::String;
use std::sync::Arc;
use utils::db::DbContext;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub url: String,
    pub param: String,
}

#[derive(Clone)]
pub struct HttpHandler {
    config: Rc<ApplicationConfig>,
    db_context: Arc<DbContext>,
    order_mgr: OrderMgr,
    project_mgr: ProjectMgr,
    transaction_mgr: TransactionMgr,
}

impl HttpHandler {
    pub fn new(app_config: &Rc<ApplicationConfig>) -> HttpHandler {
        let server_param = app_config.server_param();
        let rc_context = Arc::new(DbContext::new(
            server_param.get_db().as_str(),
            server_param.get_timezone(),
        ));

        HttpHandler {
            config: app_config.clone(),
            db_context: rc_context.clone(),
            order_mgr: OrderMgr::new(&rc_context),
            project_mgr: ProjectMgr::new(&rc_context),
            transaction_mgr: TransactionMgr::new(&app_config, &rc_context),
        }
    }

    pub fn tcp_limit_size(&self) -> u32 {
        self.config.server_param().get_tcp_limit_size()
    }

    //客户端业务请求分发
    pub fn process_request(&self, buf: &str) -> String {
        let request: Message = serde_json::from_str(buf).expect("Can't deserialize");
        debug!("request message : {:?}", request);

        let request_url = request.url.as_str();
        match request_url {
            "/order_data/lastUpdateTime" => self.order_mgr.restful_get_updated_time(),
            "/order_data/set" => self.order_mgr.restful_set_asset_order(request.param),
            "/order_data/get" => self.order_mgr.restful_get_asset_order(request.param),

            "/project_asset/lastUpdateTime" => self.project_mgr.restful_get_updated_time(ProjectTable::Asset),
            "/project_asset/set" => self.project_mgr.restful_set_project_asset(request.param),

            "/project_ledger/lastUpdateTime" => self.project_mgr.restful_get_updated_time(ProjectTable::Ledger),
            "/project_ledger/set" => self.project_mgr.restful_set_project_leger(request.param),
            "/project_ledger/get" => self.project_mgr.restful_get_project_leger(request.param),

            "/project_receipt/lastUpdateTime" => self.project_mgr.restful_get_updated_time(ProjectTable::Receipt),
            "/project_receipt/set" => self.project_mgr.restful_set_project_receipt(request.param),
            "/project_receipt/get" => self.project_mgr.restful_get_project_receipt(request.param),

            "/project_bill/create" => self.project_mgr.restful_create_bill(request.param),
            "/project_bill/get" => self.project_mgr.restful_get_project_bill(request.param),

            /*根据分账日获取待分账的账单列表*/
            "/project_bill/pay" => self.project_mgr.restful_pay_bills(request.param),
            /*查询交易未结束，需要从银联刷新状态的交易记录*/
            "/project_bill/refresh" => self.project_mgr.restful_refresh_bills(request.param),

            /*根据账单ID获取银联支付的接口参数[卡域未加密，不包含签名数据]*/
            "/unionpay/trans/param" => self.transaction_mgr.restful_trans_params(request.param),
            "/unionpay/query/param" => self.transaction_mgr.restful_query_params(request.param),

            /*调用银联分账支付的接口，由于http_req无法解决服务器信任问题，改为Java调用银联接口*/
            "/payment/record/update" => self.transaction_mgr.restful_payment_record_update(request.param),

            /*查询支付记录接口*/
            "/payment/record/get" => self.transaction_mgr.restful_payment_record_get(request.param),

            /*刷新交易结果接口*/
            "/notify" => self.transaction_mgr.restful_notify(request.param),

            "/test" => self.process_after_started(),
            _ => String::from("unknown request url, it is should do nothing")
        }
    }

    //临时方案：解决第一次操作数据库失败的问题
    fn process_after_started(&self) -> String {
        let table_num = self.db_context.table_count();
        format!("<request_url> test table number is {}.", table_num)
    }
}
