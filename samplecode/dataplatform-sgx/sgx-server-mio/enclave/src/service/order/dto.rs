use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderData {
    pub orderId: i32,
    pub assetType: i32,
    pub assetId: i32,
    pub revenue: i64,
    pub orderTime: i64,
}

impl OrderData {
    pub fn new() -> OrderData {
        OrderData {
            orderId: 0,
            assetType: 0,
            assetId: 0,
            revenue: 0,
            orderTime: 0,
        }
    }
}
