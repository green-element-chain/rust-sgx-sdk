//! 分账交易的处理模块，账单分账记录管理等
#[allow(dead_code)]
#[allow(unused_variables)]
use config::ApplicationConfig;
use std::rc::Rc;
use std::string::{String, ToString};
use std::sync::Arc;

use crate::utils::db::DbContext;

//use self::dto::*;

pub mod dto;

#[derive(Clone)]
pub struct TransactionMgr {
    app_config: Rc<ApplicationConfig>,
    db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl TransactionMgr {
    pub fn new(config: &Rc<ApplicationConfig>, context: &Arc<DbContext>) -> TransactionMgr {
        let transaction_mgr = TransactionMgr {
            app_config: config.clone(),
            db_context: context.clone(),
        };
        transaction_mgr.init_table();
        transaction_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_bill_payment (
            id integer primary key autoincrement,
            bill_id int not null,
            order_no varchar(255) not null,
            amount int not null,
            tran_method smallint not null,
            tran_time datetime null,
            status smallint not null,
            resp_code int not null,
            resp_msg varchar(255) null,
            query_times int not null,
            acq_seq_id varchar(18) null,
            create_at datetime not null default (datetime('now', 'localtime')),
            update_at datetime not null default (datetime('now', 'localtime'))
        );
        create trigger if not exists [BillPaymentLastUpdateTime]
            after update
            on project_bill_payment
            for each row
            when NEW.update_at <= OLD.update_at
        begin
            update project_bill_payment set update_at=(datetime('now', 'localtime')) where id=OLD.id;
        end;";
        self.db_context.execute(sql);
    }

    pub fn payment(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        "payment from server".to_string()
    }

    pub fn payment_b2b(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        "payment_b2b from server".to_string()
    }

    pub fn notify(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        "notify from server".to_string()
    }

    pub fn notify_b2b(&self, param: String) -> String {
        /*let client = HttpClient::new(self.config.clone());
        client.send_get("channel");*/
        "notify_b2b from server".to_string()
    }
}