//! 发起外部Restful请求的客户端，封装通用的接口供Server调用
use http_req::{
    request::{Method, Request},
    uri::Uri,
};

use std::string::String;
use std::vec::Vec;

#[derive(Clone)]
pub struct HttpClient {}

#[allow(unused_variables)]
impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {}
    }

    #[allow(dead_code)]
    pub fn send_get(&self, url: &str) -> String {
        let mut writer = Vec::new();
        let uri: Uri = url.parse().unwrap();
        let response = Request::new(&uri)
            .method(Method::GET)
            .send(&mut writer)
            .unwrap();

        let result = String::from_utf8(writer).unwrap();
        info!("Status: {} {}", response.status_code(), response.reason());
        info!("result {}", result);
        result
    }

    #[allow(dead_code)]
    pub fn send_post(&self, url: &str, param: &str) -> String {
        let mut writer = Vec::new();
        let uri: Uri = url.parse().unwrap();
        let response = Request::new(&uri)
            .method(Method::POST)
            .body(param.as_bytes())
            .send(&mut writer)
            .unwrap();

        let result = String::from_utf8(writer).unwrap();
        info!("Status: {} {}", response.status_code(), response.reason());
        info!("result {}", result);
        result
    }
}