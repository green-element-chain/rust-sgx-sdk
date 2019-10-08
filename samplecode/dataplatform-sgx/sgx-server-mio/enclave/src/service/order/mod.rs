//! 资产的订单数据管理
use chrono::NaiveDateTime;
use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteError, SqliteResult};

use service::response::{LastUpdatedTime, SgxServerResponse};
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;
use utils::db::DbContext;
use utils::time;

use self::dto::*;

pub mod dto;

#[derive(Clone)]
pub struct OrderMgr {
    db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl OrderMgr {
    pub fn new(context: &Arc<DbContext>) -> OrderMgr {
        let data_mgr = OrderMgr {
            db_context: context.clone(),
        };
        data_mgr.init_table();
        data_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists asset_order (
            order_id integer not null primary key,
            asset_type int not null,
            asset_id int not null,
            revenue int not null,
            order_time int not null,
            update_at datetime not null default (datetime('now'))
        );";
        self.db_context.exec(sql);
    }
}

// 所有的Restful接口实现
#[allow(unused_variables)]
impl OrderMgr {
    pub fn restful_get_updated_time(&self) -> String {
        let table_name = "asset_order";
        let time: Option<String> = self.db_context.get_last_update_time_local(table_name);
        LastUpdatedTime::local_updated_time_json_str(time)
    }

    pub fn restful_set_asset_order(&self, _param: String) -> String {
        let msg = "set_asset_order data to sgx server";
        let orders: Vec<OrderData> = serde_json::from_str(_param.as_str()).expect("Can't deserialize");

        let sql = "delete from asset_order";
        if !self.db_context.execute(sql) {
            return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
        }

        let update_time_at = time::now_str();
        for data in orders.iter() {
            let order_time: NaiveDateTime = time::get_time(data.orderTime as i64);
            let sql = format!("insert into asset_order(\
                order_id,asset_type,asset_id,revenue,order_time,update_at\
                ) values({},{},{},{},'{}','{}')",
                data.orderId,
                data.assetType,
                data.assetId,
                data.revenue,
                order_time.timestamp(),
                update_time_at
            );
            if !self.db_context.execute(sql.as_str()) {
                return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
            }
        }
        SgxServerResponse::success(format!("{} {}", msg, "success."))
    }

    pub fn restful_get_asset_order(&self, _param: String) -> String {
        let msg = "get_asset_order data from sgx server";
        let sql = "select order_id,asset_type,asset_id,revenue,order_time \
            from asset_order order by order_time desc limit 50";

        let statement: SqliteResult<PreparedStatement> = self.db_context.query(sql);
        if statement.is_err() {
            return SgxServerResponse::failed(format!("{} {}", msg, "failed."));
        }

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };
        let result: Result<Vec<OrderData>, SqliteError> = statement.unwrap().query_fold(
            &[], vec!(), |row, data_vec| {
                Ok(snoc(OrderData {
                    orderId: row.get(0),
                    assetType: row.get(1),
                    assetId: row.get(2),
                    revenue: row.get(3),
                    orderTime: row.get(4),
                }, data_vec))
            });
        let asset_orders: Vec<OrderData> = match result {
            Ok(v) => { v }
            Err(e) => {
                error!("failed to query asset_order {:?}", e);
                Vec::new()
            }
        };
        SgxServerResponse::success(serde_json::to_string(&asset_orders).unwrap())
    }
}