use rustls::NoClientAuth;

use config::ApplicationConfig;
use std::rc::Rc;
use utils::certauth;
use utils::file;

pub struct TlsServerConfig {
    config: Rc<ApplicationConfig>,
    tls_config: Option<rustls::ServerConfig>,
}

impl TlsServerConfig {
    pub fn new(app_config: &Rc<ApplicationConfig>) -> TlsServerConfig {
        TlsServerConfig {
            config: app_config.clone(),
            tls_config: None,
        }
    }

    pub fn get(&mut self) -> Option<rustls::ServerConfig> {
        if self.tls_config.is_none() {
            let server_param = self.config.server_param();
            if server_param.intel_cert_use() {
                println!("test");
                certauth::get_intel_cert(server_param);
                return None;
            } else {
                let certs = file::load_certs(server_param.get_cert().as_str());
                let private_key = file::load_private_key(server_param.get_key().as_str());
                let mut tls_config = rustls::ServerConfig::new(NoClientAuth::new());
                tls_config.set_single_cert_with_ocsp_and_sct(certs, private_key, vec![], vec![]).unwrap();
                self.tls_config = Some(tls_config);
            }
        }
        self.tls_config.clone()
    }
}