use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderData {
    pub orderId: u32,
    pub assetType: u16,
    pub assetId: u32,
    pub revenue: u32,
    pub orderTime: u32,
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
