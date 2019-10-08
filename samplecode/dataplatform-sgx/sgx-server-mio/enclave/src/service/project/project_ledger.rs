use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteError, SqliteResult};

use service::project::{dto::*, project_utils};
use service::response::SgxServerResponse;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

#[derive(Clone)]
pub struct ProjectLedgerMgr {
    db_context: Arc<DbContext>,
}

impl ProjectLedgerMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectLedgerMgr {
        let project_ledger_mgr = ProjectLedgerMgr {
            db_context: context.clone(),
        };
        project_ledger_mgr.init_table();
        project_ledger_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_ledger (
            project_id int not null,
            bill_date int not null,
            bill_start_date date not null,
            bill_cycle int not null,
            ledger_date int not null,
            ledger_mode int not null,
            ledger_content varchar(255) not null,
            update_at datetime not null default (datetime('now'))
        );";
        self.db_context.exec(sql);
    }

    fn inner_project_ledgers(&self, sql: &str) -> Vec<ProjectLedger> {
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
                    ledgerMode: row.get(5),
                    ledgerContent: row.get(6),
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

    pub fn get_project_ledgers_with_input(&self, param: String) -> Vec<ProjectLedger> {
        match serde_json::from_str::<ProjectBillReq>(param.as_str()) {
            Ok(ref mut req) => {
                let mut sql = format!("select * from project_ledger where bill_date = {}", req.day);
                if !req.projects.is_empty() {
                    let ids = project_utils::get_ids(&req.projects);
                    sql.push_str(format!(" and project_id in({})", ids.as_str()).as_ref());
                }
                self.inner_project_ledgers(sql.as_str())
            }
            Err(e) => {
                error!("serde_json error: {}", e);
                Vec::new()
            }
        }
    }

    pub fn set_project_leger(&self, param: String) -> String {
        let msg = "set_project_leger data to sgx server";
        let ledgers: Vec<ProjectLedger> = serde_json::from_str(param.as_str()).expect("Can't deserialize");

        let update_time_at = time::now_str();
        for data in ledgers.iter() {
            let mut sql = format!("delete from project_ledger where project_id = {}", data.projectId);
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }

            let bill_start_date = time::parse_native_date_from_str(data.billStartDate.clone());
            sql = format!("insert into project_ledger(\
                project_id,bill_date,bill_start_date,bill_cycle,ledger_date,ledger_mode,ledger_content,update_at\
                ) values({},{},'{}',{},{},{},'{}','{}')",
                data.projectId,
                data.billDate,
                bill_start_date,
                data.billCycle,
                data.ledgerDate,
                data.ledgerMode,
                data.ledgerContent,
                update_time_at,
            );
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }
        }
        SgxServerResponse::success(format!("{} {}", msg, "success."))
    }

    pub fn get_project_leger(&self, _param: String) -> String {
        let sql = "select project_id,bill_date,bill_start_date,bill_cycle,ledger_date,ledger_mode,ledger_content \
            from project_ledger order by project_id desc limit 50";

        let project_ledgers: Vec<ProjectLedger> = self.inner_project_ledgers(sql);
        SgxServerResponse::success(serde_json::to_string(&project_ledgers).unwrap())
    }
}