use std::prelude::v1::*;

#[derive(Serialize, Deserialize)]
pub struct Datatype {
    pub types: String,
    pub send_status: String,
    pub client_id: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    pub street: String,
    pub city: String,
    pub send_status: String,
    pub age: u8,
    pub client_id: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub street: String,
    pub city: String,
    pub age: u8,
    pub send_status: String,
    pub client_id: u8,
}
