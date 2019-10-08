use sqlite3::{PreparedStatement, Query, QueryFold, ResultRow, ResultRowAccess, SqliteError, SqliteResult};

use service::project::dto::*;
use std::collections::HashMap;
use std::slice::SliceConcatExt;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::channel::unionpay::{self, sdkconstants as cst};
use utils::db::DbContext;
use utils::time;

#[derive(Clone)]
pub struct ProjectBillPaymentMgr {
    db_context: Arc<DbContext>,
}

impl ProjectBillPaymentMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectBillPaymentMgr {
        let bill_payment_mgr = ProjectBillPaymentMgr {
            db_context: context.clone(),
        };
        bill_payment_mgr.init_table();
        bill_payment_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_bill_payment (
            id integer primary key autoincrement,
            order_no varchar(255) not null,
            amount int not null,
            tran_method smallint not null,
            tran_time datetime null,
            status smallint not null,
            resp_code int null,
            resp_msg varchar(255) null,
            query_times int not null default 0,
            acq_seq_id varchar(18) null,
            update_at datetime not null default (datetime('now'))
        );
        create trigger if not exists [BillPaymentLastUpdateTime]
            after update
            on project_bill_payment
            for each row
            when NEW.update_at <= OLD.update_at
        begin
            update project_bill_payment set update_at=(datetime('now')) where id=OLD.id;
        end;";
        self.db_context.execute(sql);
    }

    pub fn get_all(&self) -> Option<String> {
        let msg = "get_asset_order data from sgx server";
        let sql = "select id,order_no,amount,tran_method,tran_time,status \
            from project_bill_payment order by id asc limit 50";

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            error!("{} {}", msg, "failed.");
            return None;
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<PaymentRecord>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(PaymentRecord {
                    id: row.get(0),
                    order_no: row.get(1),
                    amount: row.get(2),
                    method: row.column_int(3) as i16,
                    tran_time: row.get(4),
                    status: row.column_int(5) as i16,
                }, data_vec))
            });
        match result {
            Ok(v) => {
                Some(serde_json::to_string(&v).unwrap())
            }
            Err(e) => {
                error!("failed to query asset_order {:?}", e);
                None
            }
        }
    }

    pub fn create(&self, param: &String, method: PaymentMethod) -> (i64, bool) {
        let msg = "create_bill data to sgx server";
        let param_map: HashMap<&str, &str> = unionpay::convert_from_json_str(param);
        let tran_date = param_map.get(cst::PARAM_TXN_DATE).unwrap();
        let tran_time = param_map.get(cst::PARAM_TXN_TIME).unwrap();

        let update_time_at = time::now_str();
        let payment_status = from_payment_status(&PaymentStatus::Processing);
        let payment_tran_time = time::parse_native_time_from_dt(tran_date, tran_time);
        let sql = format!("insert into project_bill_payment(\
                order_no,amount,tran_method,tran_time,status,update_at\
                ) values('{}',{},{},'{}',{},'{}')",
            param_map.get(cst::PARAM_ORDER_ID).unwrap(),
            param_map.get(cst::PARAM_TXN_AMT).unwrap().parse::<i64>().unwrap(),
            from_payment_method(&method),
            payment_tran_time,
            payment_status,
            update_time_at
        );

        if !self.db_context.execute(sql.as_str()) {
            error!("{} {}", msg, "failed.");
            return (0, false);
        }
        (payment_tran_time.timestamp(), true)
    }

    pub fn get_update_record(&self, order_no: &String) -> Option<PaymentUpdate> {
        let sql = format!("select pbp.id,pbp.order_no,strftime('%s',pbp.tran_time),pbp.status,pbp.query_times \
            from project_bill_payment pbp,project_bill pb where pbp.order_no=pb.order_no and pbp.tran_time=pb.tran_time \
            and pbp.order_no='{}'", order_no);
        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql.as_str());
        if statement.is_err() {
            return None;
        }
        let mut stmt = statement.unwrap();
        let mut result = stmt.query(
            &[], |row: &mut ResultRow| {
                Ok(PaymentUpdate {
                    id: row.get(0),
                    order_no: row.get(1),
                    tran_time: row.get(2),
                    status: row.column_int(3) as i16,
                    query_times: row.get(4),
                    update_time: 0,
                    resp_code: None,
                    resp_msg: None,
                    acq_seq_id: None,
                })
            });
        match result {
            Err(e) => {
                error!("failed to query project bill payment {:?}", e);
                None
            }
            Ok(ref mut v) => {
                let mut sql_result: SqliteResult<Vec<PaymentUpdate>> = v.collect();
                match sql_result {
                    Err(_) => { None }
                    Ok(ref mut list) => { list.pop() }
                }
            }
        }
    }

    pub fn update_status(&self, payment_update: &PaymentUpdate) -> bool {
        let mut sql: String;
        {
            let mut sql_vec = Vec::new();
            sql_vec.push(format!("update project_bill_payment set status={},query_times={},update_at='{}'",
                payment_update.status,
                payment_update.query_times,
                payment_update.update_time,
            ));
            if payment_update.resp_code.is_some() {
                sql_vec.push(format!(",resp_code={}", payment_update.resp_code.unwrap()));
            }
            if payment_update.resp_msg.is_some() {
                sql_vec.push(format!(",resp_msg='{}'", payment_update.resp_msg.as_ref().unwrap()));
            }
            if payment_update.acq_seq_id.is_some() {
                sql_vec.push(format!(",acq_seq_id='{}'", payment_update.acq_seq_id.as_ref().unwrap()));
            }
            sql_vec.push(format!("where id={}", payment_update.id));
            sql = sql_vec.join(" ");
        }
        self.db_context.execute(sql.as_str())
    }
}