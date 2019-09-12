//! 发起外部Restful请求的客户端，封装通用的接口供Server调用
use http_req::{
    request::{Method, RequestBuilder},
    tls,
    uri::Uri,
};
use http_req::tls::Conn;
use sgx_types::*;

use std::collections::HashMap;
use std::ffi::CString;
use std::net::TcpStream;
use std::str;
use std::string::{String, ToString};
use std::vec::Vec;

extern "C" {
    pub fn ocall_get_url_socket(ret_val: *mut sgx_status_t, req_url: *const c_uchar, socket_fd: *mut i32);
}

#[derive(Clone)]
pub struct HttpClient {}

#[allow(unused_variables)]
impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {}
    }

    fn get_request_stream(&self, url: &str, addr: &Uri) -> Result<Conn<TcpStream>, String> {
        let mut rt: sgx_status_t = sgx_status_t::SGX_ERROR_UNEXPECTED;
        let mut ias_sock: i32 = 0;

        unsafe {
            let c_str = CString::new(url).unwrap();
            ocall_get_url_socket(
                &mut rt as *mut sgx_status_t,
                c_str.as_ptr() as *const c_uchar,
                &mut ias_sock as *mut i32)
        };
        if rt != sgx_status_t::SGX_SUCCESS {
            error!("ocall_get_url_socket rt {}", rt);
            return Err(String::from("invalid socket address."));
        }

        let stream: TcpStream = TcpStream::new(ias_sock).unwrap();
        let conn_stream = tls::Config::default()
            .connect(addr.host().unwrap(), stream)
            .unwrap();

        Ok(conn_stream)
    }


    fn get_response_content(&self, result: Vec<u8>) -> String {
        match String::from_utf8(result) {
            Err(e) => {
                error!("Error: {}", e);
                e.to_string()
            }
            Ok(content) => {
                info!("Content: \n{}", content);
                content
            }
        }
    }

    pub fn send_get(&self, url: &str) -> String {
        let addr: Uri = url.parse().unwrap();
        let mut conn_stream = self.get_request_stream(url, &addr);

        let mut writer = Vec::new();
        match conn_stream {
            Err(msg) => { error!("{}", msg); }
            Ok(ref mut stream) => {
                let response = RequestBuilder::new(&addr)
                    .method(Method::GET)
                    .send(stream, &mut writer)
                    .unwrap();

                info!("Status: {} {}", response.status_code(), response.reason());
            }
        }
        self.get_response_content(writer)
    }

    pub fn send_post(&self, url: &str, param: &str) -> String {
        let addr: Uri = url.parse().unwrap();
        let mut conn_stream = self.get_request_stream(url, &addr);

        let mut writer = Vec::new();
        match conn_stream {
            Err(msg) => { error!("{}", msg); }
            Ok(ref mut stream) => {
                let response = RequestBuilder::new(&addr)
                    .method(Method::POST)
                    .header("Content-type", "text/html; charset=utf-8")
                    .body(param.as_bytes())
                    .send(stream, &mut writer)
                    .unwrap();

                info!("Status: {} {}", response.status_code(), response.reason());
            }
        }
        self.get_response_content(writer)
    }
}
