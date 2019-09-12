use chrono::{Datelike, NaiveDateTime};
use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteError, SqliteResult};

use service::project::{
    dto::*,
    project_ledger::ProjectLedgerMgr,
};
use service::response::SgxServerResponse;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

#[derive(Clone)]
pub struct ProjectBillMgr {
    db_context: Arc<DbContext>,
    ledger_mgr: ProjectLedgerMgr,
}

impl ProjectBillMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectBillMgr {
        let project_bill_mgr = ProjectBillMgr {
            db_context: context.clone(),
            ledger_mgr: ProjectLedgerMgr::new(context),
        };
        project_bill_mgr.init_table();
        project_bill_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_bill (
            id integer primary key autoincrement,
            begin_date date not null,
            end_date date not null,
            project_id int not null,
            amount int not null,
            order_no varchar(255) not null,
            serial_no varchar(18) null,
            status smallint not null,
            tran_time datetime null,
            create_at datetime not null default (datetime('now')),
            update_at datetime not null default (datetime('now'))
        );
        create trigger if not exists [ProjectBillLastUpdateTime]
            after update
            on project_bill
            for each row
            when NEW.update_at <= OLD.update_at
        begin
            update project_bill set update_at=(datetime('now')) where id=OLD.id;
        end;";
        self.db_context.exec(sql);
    }

    fn create_one_bill(&self, project_ledger: &ProjectLedger, update_time: &str) -> bool {
        let bill_time_range = self.get_bill_time_range(project_ledger.projectId as u32, project_ledger);
        if bill_time_range.is_none() {
            error!("failed to get bill date range for project {}", project_ledger.projectId);
            return false;
        }

        let date_range = bill_time_range.unwrap();
        let start_time = date_range.bill_start_time.unwrap();
        let end_time = date_range.bill_end_time.unwrap();

        let total_amount = self.get_total_amount(project_ledger.projectId as u32, start_time, end_time);
        let curr_time = time::now();
        let order_no = time::get_order_no(&curr_time);
        info!("create bill for project {}, [{} ~ {}]", project_ledger.projectId, start_time.timestamp(), end_time.timestamp());

        let bill_status = from_bill_status(BillStatus::Initialize);
        let sql = format!("insert into project_bill(\
            begin_date,end_date,project_id,amount,order_no,status,create_at,update_at\
            ) values('{}','{}',{},{},{},{},'{}','{}')",
            start_time.date(), end_time.date(),
            project_ledger.projectId,
            total_amount,
            order_no,
            bill_status,
            update_time, update_time
        );
        self.db_context.execute(sql.as_str())
    }

    fn get_bill_time_range(&self, project: u32, ledger: &ProjectLedger) -> Option<ProjectBillTimeRange> {
        let sql = format!("select end_date from project_bill where project_id = {} order by id desc limit 1", project);
        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql.as_str());
        if statement.is_err() {
            return None;
        }

        let mut start_time: Option<NaiveDateTime> = None;
        match statement.unwrap().execute().step() {
            Err(e) => { error!("failed to query project_bill {}", e); }
            Ok(None) => {}
            Ok(Some(ref mut row)) => {
                let date_value = row.column_text(0).unwrap();
                start_time = Some(time::parse_native_time_from_str(date_value));
            }
        }
        if start_time.is_none() {
            start_time = Some(time::parse_native_time_from_str(ledger.billStartDate.clone()));
        }

        if start_time.is_some() {
            let temp_time = start_time.clone().unwrap();
            let month = temp_time.month() + ledger.billCycle as u32;
            let time_range = ProjectBillTimeRange::new(start_time, temp_time.with_month(month));
            return Some(time_range);
        }
        error!("Can't find project bill time range data.");
        None
    }

    fn get_total_amount(&self, project: u32, start_time: NaiveDateTime, end_time: NaiveDateTime) -> i64 {
        let sql = format!("select sum(o.revenue) from asset_order o,project_asset a \
            where o.asset_id=a.asset_id and a.project_id = {} and o.order_time >= {} and o.order_time < {}",
            project,
            start_time.timestamp(),
            end_time.timestamp()
        );

        let mut total_amount: i64 = 0;
        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql.as_str());
        if statement.is_err() {
            return total_amount;
        }

        match statement.unwrap().execute().step() {
            Err(e) => { error!("failed to query project_ledger {}", e); }
            Ok(None) => {}
            Ok(Some(ref mut row)) => {
                total_amount = row.column_int(0) as i64;
            }
        }
        total_amount
    }

    pub fn create_bill(&self, param: String) -> String {
        let msg = "create_bill data to sgx server";

        let projects: Vec<ProjectLedger> = self.ledger_mgr.get_project_ledgers_with_input(param);
        if projects.len() == 0 {
            return SgxServerResponse::failed(
                format!("{} {}, project ledger is empty.", msg, "failed."));
        }

        let update_time_at = time::now_str();
        for data in projects.iter() {
            if !self.create_one_bill(&data, update_time_at.as_str()) {
                return SgxServerResponse::failed(
                    format!("{} {}, project {}", msg, "failed.", data.projectId));
            }
        }
        SgxServerResponse::failed(format!("{} {}", msg, "success."))
    }

    pub fn get_project_bill(&self, _param: String) -> String {
        let msg = "get_project_bill data from sgx server";
        let sql = "select id,project_id,begin_date,end_date,amount,order_no,status \
            from project_bill order by id desc limit 50";

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<ProjectBill>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(ProjectBill {
                    id: row.get(0),
                    projectId: row.get(1),
                    beginDate: row.get(2),
                    endDate: row.get(3),
                    amount: row.get(4),
                    orderNo: row.get(5),
                    status: row.get(6),
                }, data_vec))
            });
        let project_bills: Vec<ProjectBill> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query project_bill {:?}", e);
                Vec::new()
            }
        };
        SgxServerResponse::success(format!("{}", serde_json::to_string(&project_bills).unwrap()))
    }

    pub fn get_tran_params(&self, param: PaymentParam) -> Vec<TranRequestParam> {
        let mut sql = String::from("select pl.ledger_content,pl.ledger_mode,pb.order_no,pb.amount \
            from project_bill pb,project_ledger pl \
            where pb.project_id=pl.project_id where 1=1");
        if param.bill != 0 {
            let and = format!(" and pb.id = {}", param.bill);
            sql.push_str(and.as_ref());
        } else if param.day != 0 {
            let and = format!(" and pb.project_id in (select project_id from project_ledger where ledger_date={})", param.day);
            sql.push_str(and.as_ref());
        }

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql.as_str());
        if statement.is_err() {
            return Vec::new();
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<TranRequestParam>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(TranRequestParam {
                    project_id: row.get(0),
                    split_msg: row.get(1),
                    split_method: row.get(2),
                    order_no: row.get(3),
                    amount: row.get(4),
                }, data_vec))
            });
        let payment_bills: Vec<TranRequestParam> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query payment bills {:?}", e);
                Vec::new()
            }
        };

        payment_bills
    }
}