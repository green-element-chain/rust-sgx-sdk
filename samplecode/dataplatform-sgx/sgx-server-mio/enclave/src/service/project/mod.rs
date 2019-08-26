//! 项目相关数据管理，包含项目账单的生成逻辑
use chrono::{Datelike, Local, NaiveDateTime};
use sqlite3::PreparedStatement;

use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use crate::utils::db::DbContext;
use crate::utils::time;

use self::dto::*;

pub mod dto;

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
            asset_id int not null
        );
        create table if not exists project_ledger (
            project_id int not null,
            bill_date int not null,
            bill_start_date date not null,
            bill_cycle int not null,
            ledger_date int not null,
            ledger_content varchar(255) not null
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
            create_at datetime not null default (datetime('now', 'localtime')),
            update_at datetime not null default (datetime('now', 'localtime'))
        );
        create trigger if not exists [BillLastUpdateTime]
            after update
            on project_bill
            for each row
            when NEW.update_at <= OLD.update_at
        begin
            update project_bill set update_at=(datetime('now', 'localtime')) where id=OLD.id;
        end;";
        self.db_context.execute(sql);
    }

    pub fn set_project_asset(&self, param: String) -> String {
        let assets: Vec<ProjectAsset> = serde_json::from_str(param.as_str()).expect("Can't deserialize");
        for data in assets.iter() {
            let mut sql = format!("delete from project_asset where project_id = {}", data.projectId);
            self.db_context.execute(sql.as_str());

            for asset in data.assets.iter() {
                sql = format!("insert into project_asset(project_id,asset_id) values({},{})", data.projectId, asset);
                self.db_context.execute(sql.as_str());
            }
        }
        "set_project_asset from server".to_string()
    }

    pub fn set_leger(&self, param: String) -> String {
        let ledger: ProjectLedger = serde_json::from_str(param.as_str()).expect("Can't deserialize");

        let mut sql = format!("delete from project_ledger where project_id = {}", ledger.projectId);
        self.db_context.execute(sql.as_str());

        let bill_start_date = time::parse_native_date_from_str(ledger.billStartDate);
        sql = format!(
            "insert into project_ledger(project_id,bill_date,bill_start_date,bill_cycle,ledger_date,ledger_content) \
             values({},{},'{}',{},{},'{}')",
            ledger.projectId,
            ledger.billDate,
            bill_start_date,
            ledger.billCycle,
            ledger.ledgerDate,
            ledger.ledgerContent
        );
        self.db_context.execute(sql.as_str());
        "write ledger data to server".to_string()
    }

    pub fn create_bill(&self, param: String) -> String {
        let project_id: u32 = param.parse().expect("Invalid project id.");
        let ledger: ProjectLedger = self.get_project_ledger(project_id);
        debug!("{:?}", ledger);

        let bill_time_range = self.get_bill_time_range(project_id, ledger);
        if bill_time_range.is_none() {
            return "failed to create bill".to_string();
        }

        let date_range = bill_time_range.unwrap();
        let start_time = date_range.bill_start_time.unwrap();
        let end_time = date_range.bill_end_time.unwrap();

        let total_amount = self.get_total_amount(project_id, start_time, end_time);
        let curr_time = Local::now().naive_local();
        let order_no = time::get_order_no(curr_time);

        let bill_status = from_bill_status(BillStatus::Initialize);
        let sql = format!(
            "insert into project_bill(begin_date,end_date,project_id,amount,order_no,status) \
             values('{}','{}',{},{},{},{})",
            start_time, end_time,
            project_id,
            total_amount,
            order_no,
            bill_status
        );
        self.db_context.execute(sql.as_str());
        "create_bill from server".to_string()
    }

    fn get_bill_time_range(&self, project: u32, ledger: ProjectLedger) -> Option<ProjectBillTimeRange> {
        let mut start_time: Option<NaiveDateTime> = None;
        let sql = format!("select end_date from project_bill where project_id = {} order by id desc limit 1", project);
        let mut statement: PreparedStatement = self.db_context.query(sql.as_str());
        match statement.execute().step() {
            Err(e) => { error!("failed to query project_bill {}", e); }
            Ok(None) => {}
            Ok(Some(ref mut row)) => {
                let date_value = row.column_text(0).unwrap();
                start_time = Some(time::parse_native_time_from_str(date_value));
            }
        }
        if start_time.is_none() {
            start_time = Some(time::parse_native_time_from_str(ledger.billStartDate));
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

    pub fn get_project_ledger(&self, project: u32) -> ProjectLedger {
        let mut resp_data: ProjectLedger = ProjectLedger::new();
        let sql = format!("select * from project_ledger where project_id = {}", project);
        let mut statement: PreparedStatement = self.db_context.query(sql.as_str());
        match statement.execute().step() {
            Err(e) => { error!("failed to query project_ledger {}", e); }
            Ok(None) => {}
            Ok(Some(ref mut row)) => {
                resp_data.projectId = row.column_int(0) as u32;
                resp_data.billDate = row.column_int(1) as u8;
                resp_data.billStartDate = row.column_text(2).unwrap();
                resp_data.billCycle = row.column_int(3) as u8;
                resp_data.ledgerDate = row.column_int(4) as u16;
                resp_data.ledgerContent = row.column_text(5).unwrap();
            }
        }
        resp_data
    }

    fn get_total_amount(&self, project: u32, start_time: NaiveDateTime, end_time: NaiveDateTime) -> i64 {
        let sql = format!("select sum(o.revenue) from asset_order o,project_asset a \
            where o.asset_id=a.asset_id and a.project_id = {} and o.order_time >= '{}' and o.order_time < '{}'",
            project,
            start_time,
            end_time
        );

        let mut total_amount: i64 = 0;
        let mut statement: PreparedStatement = self.db_context.query(sql.as_str());
        match statement.execute().step() {
            Err(e) => { error!("failed to query project_ledger {}", e); }
            Ok(None) => {}
            Ok(Some(ref mut row)) => {
                total_amount = row.column_int(0) as i64;
            }
        }
        total_amount
    }
}