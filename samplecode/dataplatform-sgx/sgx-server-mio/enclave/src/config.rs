use serde_derive::Deserialize;

use std::clone::Clone;
use std::io::Read;
use std::string::String;
use std::untrusted::fs::File;
use std::vec::Vec;

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

    pub fn log_level(&self) -> String {
        self.log_level.clone().unwrap_or(String::from(""))
    }

    pub fn intel_cert_use(&self) -> bool {
        self.intel_cert_use.unwrap_or(false)
    }

    pub fn get_intel_ca(&self) -> String {
        self.intel_ca.clone().unwrap()
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

    pub fn tran_url(&self) -> String {
        self.back_url.clone().unwrap()
    }

    pub fn tran_url_b2b(&self) -> String {
        self.front_url.clone().unwrap()
    }

    pub fn query_url(&self) -> String {
        self.query_url.clone().unwrap()
    }

    pub fn sms_url(&self) -> String {
        self.sms_url.clone().unwrap()
    }

    pub fn mer_url(&self) -> String {
        self.merchant_url.clone().unwrap()
    }

    pub fn mer_query_url(&self) -> String {
        self.merchant_query_url.clone().unwrap()
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
    signature_field: Option<String>,
    signature_exclude: Option<String>,
    signature_cert_type: Option<String>,
    signature_cert_password: Option<String>,
    signature_cert_tran: Option<String>,
    signature_cert_verify: Option<String>,
}

impl UnionpayTranParam {
    pub fn tran_type_b2b(&self) -> String {
        self.tran_type_b2b.clone().unwrap()
    }

    pub fn mer_id(&self) -> String {
        self.tran_mer_id.clone().unwrap()
    }

    pub fn redirect_url(&self) -> String {
        self.tran_redirect.clone().unwrap()
    }

    pub fn notify_tran_url_b2b(&self) -> String {
        self.notify_front.clone().unwrap()
    }

    pub fn notify_tran_url(&self) -> String {
        self.notify_back.clone().unwrap()
    }

    pub fn notify_sign_url(&self) -> String {
        self.notify_sign.clone().unwrap()
    }

    pub fn remote_ip(&self) -> String {
        self.remote_ip.clone().unwrap()
    }

    pub fn sign_field(&self) -> String {
        self.signature_field.clone().unwrap()
    }

    pub fn sign_exclude(&self) -> String {
        self.signature_exclude.clone().unwrap()
    }

    pub fn sign_cert_type(&self) -> String {
        self.signature_cert_type.clone().unwrap()
    }

    pub fn sign_cert_pwd(&self) -> String {
        self.signature_cert_password.clone().unwrap()
    }

    pub fn sign_cert_tran(&self) -> String {
        self.signature_cert_tran.clone().unwrap()
    }

    pub fn sign_cert_verify(&self) -> String {
        self.signature_cert_verify.clone().unwrap()
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
        fn update_string(parent_path: &String, object: Option<String>) -> Option<String> {
            let cfg_value = object.unwrap_or(String::from(""));
            if !cfg_value.is_empty() {
                let real_value = parent_path.clone() + cfg_value.as_str();
                return Some(real_value);
            }
            Some(cfg_value.parse().unwrap())
        }

        let server_params = self.server_param.as_mut().unwrap();
        for x in server_params {
            x.tls_cert = update_string(relative_path, x.tls_cert.clone());
            x.tls_key = update_string(relative_path, x.tls_key.clone());
            x.db_store = update_string(relative_path, x.db_store.clone());
        }

        let tran_params = self.unionpay_tran_param.as_mut().unwrap();
        for y in tran_params {
            y.signature_cert_tran = update_string(relative_path, y.signature_cert_tran.clone());
            y.signature_cert_verify = update_string(relative_path, y.signature_cert_verify.clone());
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
