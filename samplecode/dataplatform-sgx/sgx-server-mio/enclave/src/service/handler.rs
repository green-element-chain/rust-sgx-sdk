use serde_derive::{Deserialize, Serialize};

use config::ApplicationConfig;
use service::bill::BillMgr;
use service::order::OrderMgr;
use service::project::ProjectMgr;
use service::transaction::TransactionMgr;
use std::rc::Rc;
use std::string::{String, ToString};
use std::sync::Arc;
use utils::db::DbContext;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub url: String,
    pub param: String,
}

#[derive(Clone)]
pub struct HttpHandler {
    order_mgr: OrderMgr,
    project_mgr: ProjectMgr,
    bill_mgr: BillMgr,
    transaction_mgr: TransactionMgr,
}

impl HttpHandler {
    pub fn new(app_config: &Rc<ApplicationConfig>) -> HttpHandler {
        let db_file = app_config.server_param().get_db();
        let rc_context = Arc::new(DbContext::new(db_file.as_str()));

        let handler = HttpHandler {
            order_mgr: OrderMgr::new(&rc_context),
            project_mgr: ProjectMgr::new(&rc_context),
            bill_mgr: BillMgr::new(&rc_context),
            transaction_mgr: TransactionMgr::new(&app_config, &rc_context),
        };
        handler
    }

    //客户端业务请求分发
    pub fn process_request(&self, buf: &str) -> String {
        let request: Message = serde_json::from_str(buf).expect("Can't deserialize");
        debug!("request message : {:?}", request);

        let request_url = request.url.as_str();
        match request_url {
            "/order_data/get" => self.order_mgr.get_order(request.param),
            "/order_data/set" => self.order_mgr.set_order(request.param),

            "/project_asset/set" => self.project_mgr.set_project_asset(request.param),
            "/project_ledger/set" => self.project_mgr.set_leger(request.param),
            "/project_bill/create" => self.project_mgr.create_bill(request.param),

            "/payment" => self.transaction_mgr.payment(request.param),
            "/payment/b2b" => self.transaction_mgr.payment_b2b(request.param),

            "/notify" => self.transaction_mgr.notify(request.param),
            "/notify/b2b" => self.transaction_mgr.notify_b2b(request.param),
            _ => "unknown request url, it is should do nothing".to_string(),
        }
    }
}
