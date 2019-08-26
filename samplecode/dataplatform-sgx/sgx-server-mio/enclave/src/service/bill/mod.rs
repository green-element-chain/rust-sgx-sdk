//! 项目账单的信息更新
use utils::db::DbContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct BillMgr {
    db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl BillMgr {
    pub fn new(context: &Arc<DbContext>) -> BillMgr {
        let bill_mgr = BillMgr {
            db_context: context.clone(),
        };
        bill_mgr.init_table();
        bill_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists asset_order (
            order_id int not null primary key,
            asset_type int not null,
            asset_id int not null,
            revenue int not null
        );";
        self.db_context.execute(sql);
    }
}
