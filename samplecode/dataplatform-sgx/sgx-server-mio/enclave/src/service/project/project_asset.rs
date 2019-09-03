use service::project::dto::ProjectAsset;
use service::response::SgxServerResponse;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

#[derive(Clone)]
pub struct ProjectAssetMgr {
    db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl ProjectAssetMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectAssetMgr {
        let project_asset_mgr = ProjectAssetMgr {
            db_context: context.clone(),
        };
        project_asset_mgr.init_table();
        project_asset_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_asset (
            project_id int not null,
            asset_id int not null,
            update_at datetime not null default (datetime('now'))
        );";
        self.db_context.exec(sql);
    }

    pub fn set_project_asset(&self, param: String) -> String {
        let msg = "set_project_asset data to sgx server";
        let assets: Vec<ProjectAsset> = serde_json::from_str(param.as_str()).expect("Can't deserialize");

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
        SgxServerResponse::success(format!("{} {}", msg, "success."))
    }
}