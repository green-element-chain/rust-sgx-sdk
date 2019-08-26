//! 资产的订单数据管理
use chrono::NaiveDateTime;
use sqlite3::{PreparedStatement, QueryFold};

use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use crate::utils::db::DbContext;
use crate::utils::time;

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
            order_id int not null primary key,
            asset_type int not null,
            asset_id int not null,
            revenue int not null,
            order_time datetime not null
        );";
        self.db_context.execute(sql);
    }

    pub fn get_order(&self, param: String) -> String {
        let mut data_vec: Vec<OrderData> = Vec::new();
        let mut statement: PreparedStatement = self.db_context.query("select * from asset_order");
        loop {
            match statement.execute().step() {
                Err(e) => {
                    error!("failed to query asset_order {}", e);
                    break;
                }
                Ok(None) => { break; }
                Ok(Some(ref mut row)) => {
                    let mut data: OrderData = OrderData::new();
                    data.assetId = row.column_int(0) as u32;
                    data.assetType = row.column_int(1) as u16;
                    data.assetId = row.column_int(2) as u32;
                    data.revenue = row.column_int(3) as u32;
                    data.orderTime = row.column_int(4) as u32;
                    data_vec.push(data);
                }
            }
        }
        let data_string = serde_json::to_string(&data_vec).unwrap();
        let resp_data = format!("read order data from server:({})\r\n{}", data_vec.len(), data_string);
        resp_data
    }

    pub fn set_order(&self, param: String) -> String {
        let orders: Vec<OrderData> = serde_json::from_str(param.as_str()).expect("Can't deserialize");
        for data in orders.iter() {
            let order_time: NaiveDateTime = time::parse_native_time_from_nanosecond(data.orderTime);
            let sql = format!(
                "insert into asset_order(order_id,asset_type,asset_id,revenue,order_time) values({},{},{},{},'{}')",
                data.orderId,
                data.assetType,
                data.assetId,
                data.revenue,
                order_time
            );
            self.db_context.execute(sql.as_str());
        }
        "write order data to server".to_string()
    }
}