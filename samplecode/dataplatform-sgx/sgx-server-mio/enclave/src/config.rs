use serde_derive::Deserialize;

use std::clone::Clone;
use std::io::Read;
use std::string::{String,ToString};
use std::untrusted::fs::File;
use std::vec::Vec;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct ServerParam {
    port: Option<u32>,
    tcp_limit_size: Option<u32>,
    log_level: Option<String>,
    intel_cert_use: Option<bool>,
    intel_ca: Option<String>,
    tls_cert: Option<String>,
    tls_key: Option<String>,
    db_store: Option<String>,
    db_conn_max: Option<u16>,
    db_timezone: Option<i32>,
}

impl ServerParam {
    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(3443)
    }

    pub fn get_tcp_limit_size(&self) -> u32 {
        self.tcp_limit_size.unwrap_or(10360)
    }

    pub fn log_level(&self) -> &String {
        self.log_level.as_ref().unwrap()
    }

    pub fn intel_cert_use(&self) -> bool {
        self.intel_cert_use.unwrap_or(false)
    }

    pub fn get_intel_ca(&self) -> &String {
        self.intel_ca.as_ref().unwrap()
    }

    pub fn get_cert(&self) -> &String {
        self.tls_cert.as_ref().unwrap()
    }

    pub fn get_key(&self) -> &String {
        self.tls_key.as_ref().unwrap()
    }

    pub fn get_db(&self) -> &String {
        self.db_store.as_ref().unwrap()
    }

    pub fn get_db_max_conn(&self) -> u16 {
        self.db_conn_max.unwrap_or(30)
    }

    pub fn get_timezone(&self) -> i32 {
        self.db_timezone.unwrap_or(28800)
    }
}

#[derive(Deserialize, Debug)]
pub struct UnionpayTranUrl {
    test_url: Option<bool>,
    front_url: Option<String>,
    back_url: Option<String>,
    query_url: Option<String>,
    sms_url: Option<String>,
    merchant_url: Option<String>,
    merchant_query_url: Option<String>,
}

impl UnionpayTranUrl {
    pub fn is_test(&self) -> bool {
        self.test_url.unwrap()
    }

    pub fn tran_url(&self) -> &String {
        self.back_url.as_ref().unwrap()
    }

    pub fn tran_url_b2b(&self) -> &String {
        self.front_url.as_ref().unwrap()
    }

    pub fn query_url(&self) -> &String {
        self.query_url.as_ref().unwrap()
    }

    pub fn sms_url(&self) -> &String {
        self.sms_url.as_ref().unwrap()
    }

    pub fn mer_url(&self) -> &String {
        self.merchant_url.as_ref().unwrap()
    }

    pub fn mer_query_url(&self) -> &String {
        self.merchant_query_url.as_ref().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct UnionpayTranParam {
    tran_type_b2b: Option<String>,
    tran_mer_id: Option<String>,
    tran_redirect: Option<String>,
    notify_front: Option<String>,
    notify_back: Option<String>,
    notify_sign: Option<String>,
    remote_ip: Option<String>,
}

impl UnionpayTranParam {
    pub fn tran_type_b2b(&self) -> &String {
        self.tran_type_b2b.as_ref().unwrap()
    }

    pub fn mer_id(&self) -> &String {
        self.tran_mer_id.as_ref().unwrap()
    }

    pub fn redirect_url(&self) -> &String {
        self.tran_redirect.as_ref().unwrap()
    }

    pub fn notify_tran_url_b2b(&self) -> &String {
        self.notify_front.as_ref().unwrap()
    }

    pub fn notify_tran_url(&self) -> &String {
        self.notify_back.as_ref().unwrap()
    }

    pub fn notify_sign_url(&self) -> &String {
        self.notify_sign.as_ref().unwrap()
    }

    pub fn remote_ip(&self) -> &String {
        self.remote_ip.as_ref().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    server_param: Option<Vec<ServerParam>>,
    unionpay_tran_url: Option<Vec<UnionpayTranUrl>>,
    unionpay_tran_param: Option<Vec<UnionpayTranParam>>,
}

impl ApplicationConfig {
    pub fn new(file: &str, relative_path: String) -> ApplicationConfig {
        let mut file = File::open(file).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file content");

        let mut config: ApplicationConfig = toml::from_str(&contents).unwrap();
        config.set_relative_path(&relative_path);
        config
    }

    fn set_relative_path(&mut self, relative_path: &String) {
        fn exists(object: Option<String>) -> bool {
            if object.is_some() {
                let cfg_value = object.unwrap();
                if !cfg_value.is_empty() {
                    return Path::new(cfg_value.as_str()).exists();
                }
            }
            false
        }

        fn update_string(parent_path: &String, object: Option<String>) -> Option<String> {
            let cfg_value = object.unwrap_or(String::from(""));
            if !cfg_value.is_empty() {
                let real_value = parent_path.to_string() + cfg_value.as_str();
                return Some(real_value);
            }
            Some(cfg_value.parse().unwrap())
        }

        let server_params = self.server_param.as_mut().unwrap();
        for x in server_params {
            x.tls_cert = update_string(relative_path, x.tls_cert.clone());
            x.tls_key = update_string(relative_path, x.tls_key.clone());
            if !exists(x.db_store.clone()) {
                x.db_store = update_string(relative_path, x.db_store.clone());
            }
        }
    }

    pub fn server_param(&self) -> &ServerParam {
        self.server_param.as_ref().unwrap().get(0).unwrap()
    }

    pub fn unionpay_tran_url(&self) -> &UnionpayTranUrl {
        let urls = self.unionpay_tran_url.as_ref().unwrap();
        for x in urls.iter() {
            if x.is_test() {
                return x;
            }
        }
        panic!("Invalid tranUrl, please check unionpay configure.")
    }

    pub fn unionpay_tran_param(&self) -> &UnionpayTranParam {
        self.unionpay_tran_param.as_ref().unwrap().get(0).unwrap()
    }
}
