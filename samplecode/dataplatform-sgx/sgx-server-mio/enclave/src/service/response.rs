use serde_derive::{Deserialize, Serialize};

use std::string::String;
use utils::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct SgxServerResponse {
    success: bool,
    //失败了才需要填写Message
    message: String,
    //成功了需要填写成功的数据
    data: String,
}

impl SgxServerResponse {
    fn new(_success: bool) -> SgxServerResponse {
        SgxServerResponse {
            success: _success,
            message: String::from(""),
            data: String::from(""),
        }
    }

    fn set_data(&mut self, _data: String) {
        self.data = _data;
    }

    fn set_message(&mut self, _message: String) {
        self.message = _message;
    }
}

impl SgxServerResponse {
    pub fn success_ext(_data: String, _message: String) -> String {
        let mut response = SgxServerResponse::new(true);
        response.set_data(_data);
        response.set_message(_message);
        serde_json::to_string(&response).unwrap()
    }

    pub fn success(_data: String) -> String {
        let mut response = SgxServerResponse::new(true);
        response.set_data(_data);
        serde_json::to_string(&response).unwrap()
    }

    pub fn failed(_message: String) -> String {
        error!("{}", _message.as_str());
        let mut response = SgxServerResponse::new(false);
        response.set_message(_message);
        serde_json::to_string(&response).unwrap()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LastUpdatedTime {
    lastTime: i64,
}

impl LastUpdatedTime {
    fn new(_time: i64) -> LastUpdatedTime {
        LastUpdatedTime {
            lastTime: _time,
        }
    }
}

impl LastUpdatedTime {
    pub fn response_json_str(_time: i64) -> String {
        let data = LastUpdatedTime::new(_time);
        serde_json::to_string(&data).unwrap()
    }

    pub fn local_updated_time_json_str(_time: Option<String>) -> String {
        match _time {
            None => { SgxServerResponse::failed(String::from("failed to get last updated time.")) }
            Some(r) => {
                let mut updated_time: i64 = 0;
                if !r.is_empty() {
                    let date_time = time::parse_native_time_from_str(r);
                    updated_time = date_time.timestamp();
                }
                let data_str = LastUpdatedTime::response_json_str(updated_time);
                SgxServerResponse::success(data_str)
            }
        }
    }
}