//! 项目相关数据管理，包含项目账单的生成逻辑
use chrono::{Datelike, NaiveDateTime};
use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteError, SqliteResult};

use service::response::{LastUpdatedTime, SgxServerResponse};
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

use self::dto::*;

pub mod dto;

pub enum ProjectTable {
    Asset,
    Bill,
    Ledger,
}

#[derive(Clone)]
pub struct ProjectMgr {
    db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl ProjectMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectMgr {
        let project_mgr = ProjectMgr {
            db_context: context.clone(),
        };
        project_mgr.init_table();
        project_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_asset (
            project_id int not null,
            asset_id int not null,
            update_at datetime not null default (datetime('now'))
        );

        create table if not exists project_ledger (
            project_id int not null,
            bill_date int not null,
            bill_start_date date not null,
            bill_cycle int not null,
            ledger_date int not null,
            ledger_content varchar(255) not null,
            update_at datetime not null default (datetime('now'))
        );

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
        end;
        ";
        self.db_context.exec(sql);
    }

    pub fn restful_get_updated_time(&self, tbl: ProjectTable) -> String {
        let table_name = match tbl {
            ProjectTable::Asset => "project_asset",
            ProjectTable::Bill => "project_bill",
            ProjectTable::Ledger => "project_ledger",
        };
        let time: Option<String> = self.db_context.get_last_update_time_local(table_name);
        LastUpdatedTime::local_updated_time_json_str(time)
    }

    pub fn restful_set_project_asset(&self, _param: String) -> String {
        let msg = "set_project_asset data to sgx server";
        let assets: Vec<ProjectAsset> = serde_json::from_str(_param.as_str()).expect("Can't deserialize");

        let update_time_at = time::now_str();
        for data in assets.iter() {
            let mut sql = format!("delete from project_asset where project_id = {}", data.projectId);
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }

            for asset in data.assets.iter() {
                sql = format!("insert into project_asset(\
                    project_id,asset_id,update_at\
                    ) values({},{},'{}')",
                    data.projectId,
                    asset,
                    update_time_at
                );
                if !self.db_context.execute(sql.as_str()) {
                    return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
                }
            }
        }
        return SgxServerResponse::success(format!("{} {}", msg, "success."));
    }

    pub fn restful_set_project_leger(&self, _param: String) -> String {
        let msg = "set_project_leger data to sgx server";
        let ledgers: Vec<ProjectLedger> = serde_json::from_str(_param.as_str()).expect("Can't deserialize");

        let update_time_at = time::now_str();
        for data in ledgers.iter() {
            let mut sql = format!("delete from project_ledger where project_id = {}", data.projectId);
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }

            let bill_start_date = time::parse_native_date_from_str(data.billStartDate.clone());
            sql = format!("insert into project_ledger(\
                project_id,bill_date,bill_start_date,bill_cycle,ledger_date,ledger_content,update_at\
                ) values({},{},'{}',{},{},'{}','{}')",
                data.projectId,
                data.billDate,
                bill_start_date,
                data.billCycle,
                data.ledgerDate,
                data.ledgerContent,
                update_time_at,
            );
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }
        }
        return SgxServerResponse::success(format!("{} {}", msg, "success."));
    }

    pub fn restful_get_project_leger(&self, _param: String) -> String {
        let msg = "get_project_leger data from sgx server";
        let sql = "select project_id,bill_date,bill_start_date,bill_cycle,ledger_date,ledger_content \
            from project_ledger order by project_id desc limit 50";

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<ProjectLedger>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(ProjectLedger {
                    projectId: row.get(0),
                    billDate: row.get(1),
                    billStartDate: row.get(2),
                    billCycle: row.get(3),
                    ledgerDate: row.get(4),
                    ledgerContent: row.get(5),
                }, data_vec))
            });
        let project_ledgers: Vec<ProjectLedger> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query project_ledger {:?}", e);
                Vec::new()
            }
        };
        return SgxServerResponse::success(format!("{}", serde_json::to_string(&project_ledgers).unwrap()));
    }

    pub fn restful_create_bill(&self, _param: String) -> String {
        let projects: Vec<ProjectLedger>;
        match serde_json::from_str::<Vec<ProjectID>>(_param.as_str()) {
            Ok(ref mut v) => {
                let ids = self.get_ids(v);
                let sql = format!("select * from project_ledger where project_id in({})", ids.as_str());
                projects = self.get_project_ledgers(sql.as_str());
            }
            Err(_) => {
                let sql = "select * from project_ledger";
                projects = self.get_project_ledgers(sql);
            }
        }

        let msg = "create_bill data to sgx server";
        if projects.len() == 0 {
            return SgxServerResponse::failed(
                format!("{} {}, project ledger is empty.", msg, "failed."));
        }

        let update_time_at = time::now_str();
        for data in projects.iter() {
            if !self.create_bill(&data, update_time_at.as_str()) {
                return SgxServerResponse::failed(
                    format!("{} {}, project {}", msg, "failed.", data.projectId));
            }
        }
        return SgxServerResponse::failed(format!("{} {}", msg, "success."));
    }

    fn get_ids(&self, projects: &Vec<ProjectID>) -> String {
        let mut ids = String::new();

        let mut index: i32 = 0;
        let last_pos: i32 = (projects.len() - 1) as i32;
        for x in projects.iter() {
            ids.push_str(format!("{}", x.id).as_str());
            if index != last_pos {
                ids.push_str(",");
            }
            index += 1;
        }
        ids
    }

    pub fn get_project_ledgers(&self, sql: &str) -> Vec<ProjectLedger> {
        info!("get project ledger sql: {}", sql);
        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            return Vec::new();
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<ProjectLedger>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(ProjectLedger {
                    projectId: row.get(0),
                    billDate: row.get(1),
                    billStartDate: row.get(2),
                    billCycle: row.get(3),
                    ledgerDate: row.get(4),
                    ledgerContent: row.get(5),
                }, data_vec))
            });
        let project_ledgers: Vec<ProjectLedger> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query project_ledger {:?}", e);
                Vec::new()
            }
        };
        project_ledgers
    }

    fn create_bill(&self, project_ledger: &ProjectLedger, update_time: &str) -> bool {
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

    pub fn restful_get_project_bill(&self, _param: String) -> String {
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
        return SgxServerResponse::success(format!("{}", serde_json::to_string(&project_bills).unwrap()));
    }
}