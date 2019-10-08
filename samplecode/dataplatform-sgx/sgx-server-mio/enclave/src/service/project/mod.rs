//! 项目相关数据管理，包含项目账单的生成逻辑
use service::project::{
    project_asset::ProjectAssetMgr,
    project_bill::ProjectBillMgr,
    project_ledger::ProjectLedgerMgr,
    project_receipt::ProjectReceiptMgr,
    project_utils::ProjectTable,
};
use service::response::SgxServerResponse;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;

pub mod dto;
mod project_asset;
pub mod project_bill;
pub mod project_bill_payment;
pub mod project_ledger;
pub mod project_receipt;
pub mod project_utils;

#[derive(Clone)]
pub struct ProjectMgr {
    db_context: Arc<DbContext>,
    asset_mgr: ProjectAssetMgr,
    ledger_mgr: ProjectLedgerMgr,
    receipt_mgr: ProjectReceiptMgr,
    bill_mgr: ProjectBillMgr,
}

#[allow(unused_variables)]
impl ProjectMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectMgr {
        let project_mgr = ProjectMgr {
            db_context: context.clone(),
            asset_mgr: ProjectAssetMgr::new(context),
            ledger_mgr: ProjectLedgerMgr::new(context),
            receipt_mgr: ProjectReceiptMgr::new(context),
            bill_mgr: ProjectBillMgr::new(context),
        };
        project_mgr
    }
}

// 所有的Restful接口实现
#[allow(unused_variables)]
impl ProjectMgr {
    pub fn restful_get_updated_time(&self, tbl: ProjectTable) -> String {
        project_utils::get_updated_time(&self.db_context, &tbl)
    }

    pub fn restful_set_project_asset(&self, param: String) -> String {
        self.asset_mgr.set_project_asset(param)
    }

    pub fn restful_set_project_leger(&self, param: String) -> String {
        self.ledger_mgr.set_project_leger(param)
    }

    pub fn restful_get_project_leger(&self, param: String) -> String {
        self.ledger_mgr.get_project_leger(param)
    }

    pub fn restful_set_project_receipt(&self, param: String) -> String {
        self.receipt_mgr.set_project_receipt(param)
    }

    pub fn restful_get_project_receipt(&self, param: String) -> String {
        self.receipt_mgr.get_project_receipts(param)
    }

    pub fn restful_create_bill(&self, param: String) -> String {
        self.bill_mgr.create_bill(param)
    }

    pub fn restful_get_project_bill(&self, param: String) -> String {
        self.bill_mgr.get_project_bill(param)
    }

    pub fn restful_pay_bills(&self, param: String) -> String {
        let ledger_day: u32 = param.parse::<u32>().unwrap();
        let project_bills: Vec<u32> = self.bill_mgr.get_pay_bills(ledger_day);

        SgxServerResponse::success(serde_json::to_string(&project_bills).unwrap())
    }

    pub fn restful_refresh_bills(&self, _param: String) -> String {
        let project_bills: Vec<u32> = self.bill_mgr.get_refresh_status_bills();
        SgxServerResponse::success(serde_json::to_string(&project_bills).unwrap())
    }
}