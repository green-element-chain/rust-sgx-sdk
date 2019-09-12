use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteError, SqliteResult};

use service::project::dto::ProjectReceipt;
use service::response::SgxServerResponse;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

#[derive(Clone)]
pub struct ProjectReceiptMgr {
    db_context: Arc<DbContext>,
}

impl ProjectReceiptMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectReceiptMgr {
        let project_receipt_mgr = ProjectReceiptMgr {
            db_context: context.clone(),
        };
        project_receipt_mgr.init_table();
        project_receipt_mgr
    }

    fn init_table(&self) {
        //支付模式charge_model：0为代收，1为B2B
        let sql = "
        create table if not exists project_receipt (
            project_id int not null,
            charge_model int not null,
            card_num varchar(255) null,
            card_user varchar(20) null,
            cert_type int null,
            cert_no varchar(50) null,
            mobile varchar(12) null,
            sign_status smallint not null default 0,
            sign_order_no varchar(32) null,
            sign_resp_code int null,
            update_at datetime not null default (datetime('now'))
        );";
        self.db_context.exec(sql);
    }

    pub fn set_project_receipt(&self, param: String) -> String {
        let msg = "set_project_receipt data to sgx server";
        let receipts: Vec<ProjectReceipt> = serde_json::from_str(param.as_str()).expect("Can't deserialize");

        let update_time_at = time::now_str();
        for data in receipts.iter() {
            let mut sql = format!("delete from project_receipt where project_id = {}", data.projectId);
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }

            sql = format!("insert into project_receipt(\
                project_id,charge_model,card_num,card_user,cert_type,cert_no,mobile,update_at\
                ) values({},{},'{}','{}',{},'{}','{}','{}')",
                data.projectId,
                data.chargeMode,
                data.cardNum,
                data.cardUser,
                data.certType,
                data.certNo,
                data.mobile,
                update_time_at,
            );
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }
        }
        SgxServerResponse::success(format!("{} {}", msg, "success."))
    }

    pub fn get_project_receipt(&self, _param: String) -> String {
        let msg = "get_project_receipt data from sgx server";
        let sql = "select project_id,charge_model,card_num,card_user,cert_type,cert_no,mobile \
            from project_receipt order by project_id desc limit 50";

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<ProjectReceipt>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(ProjectReceipt {
                    projectId: row.get(0),
                    chargeMode: row.get(1),
                    cardNum: row.get(2),
                    cardUser: row.get(3),
                    certType: row.get(4),
                    certNo: row.get(5),
                    mobile: row.get(6),
                }, data_vec))
            });
        let project_receipts: Vec<ProjectReceipt> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query project_receipt {:?}", e);
                Vec::new()
            }
        };
        SgxServerResponse::success(format!("{}", serde_json::to_string(&project_receipts).unwrap()))
    }

    pub fn get_project_receipt_one(&self, project: u32) -> Option<ProjectReceipt> {
        None
    }
}