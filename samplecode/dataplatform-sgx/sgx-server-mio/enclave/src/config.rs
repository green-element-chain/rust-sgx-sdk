use serde_derive::Deserialize;

use std::clone::Clone;
use std::io::Read;
use std::string::String;
use std::untrusted::fs::File;
use std::vec::Vec;

#[derive(Deserialize, Debug)]
pub struct ServerParam {
    port: Option<u32>,
    log_level: Option<String>,
    intel_cert_use: Option<bool>,
    intel_ca: Option<String>,
    tls_cert: Option<String>,
    tls_key: Option<String>,
    db_store: Option<String>,
    db_conn_max: Option<u16>,
}

impl ServerParam {
    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(3443)
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
}

#[derive(Deserialize, Debug)]
pub struct UnionpayTranUrl {
    test_url: Option<bool>,
    pub front_url: Option<String>,
    pub back_url: Option<String>,
    pub query_url: Option<String>,
    pub sms_url: Option<String>,
    pub merchant_url: Option<String>,
    pub merchant_query_url: Option<String>,
}

impl UnionpayTranUrl {
    pub fn is_test(&self) -> bool {
        self.test_url.unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct UnionpayTranParam {
    use_test_url: Option<bool>,
    pub tran_front_type: Option<String>,
    pub tran_mer_id: Option<String>,
    pub tran_redirect: Option<String>,
    pub notify_front: Option<String>,
    pub notify_back: Option<String>,
    pub notify_sign: Option<String>,
    pub remote_ip: Option<String>,
    pub signature_field: Option<String>,
    pub signature_exclude: Option<String>,
    pub signature_cert_type: Option<String>,
    pub signature_cert_password: Option<String>,
    pub signature_cert_tran: Option<String>,
    pub signature_cert_verify: Option<String>,
}

impl UnionpayTranParam {
    pub fn is_test(&self) -> bool {
        self.use_test_url.unwrap()
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

    pub fn get_tran_url(&self) -> Option<&UnionpayTranUrl> {
        let tran_param = self.get_tran_param();
        let use_for_test: bool = tran_param.is_test();

        let urls = self.unionpay_tran_url.as_ref().unwrap();
        for x in urls {
            if x.is_test() == use_for_test {
                return Some(x);
            }
        }
        None
    }

    pub fn get_tran_param(&self) -> &UnionpayTranParam {
        self.unionpay_tran_param.as_ref().unwrap().get(0).unwrap()
    }
}
