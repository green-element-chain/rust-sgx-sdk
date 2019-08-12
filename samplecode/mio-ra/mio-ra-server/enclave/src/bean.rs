use std::prelude::v1::*;

#[derive(Serialize, Default, Clone, Deserialize)]
pub struct Teacher {
    pub street: String,
    pub city: String,
    pub send_status: String,
    pub data_type: String,
    pub ops: String,
    pub age: u8,
    pub client_id: u32,
    pub index: u32,
}

#[derive(Serialize, Default, Clone, Deserialize)]
pub struct Student {
    pub street: String,
    pub city: String,
    pub data_type: String,
    pub ops: String,
    pub age: u8,
    pub send_status: String,
    pub client_id: u32,
    pub index: u32,
}
