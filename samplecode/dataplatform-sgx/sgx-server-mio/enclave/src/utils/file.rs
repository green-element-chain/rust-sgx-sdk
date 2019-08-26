use std::io::BufReader;
use std::string::{String, ToString};
use std::untrusted::fs;
use std::vec::Vec;

pub fn get_application_config() -> (String, String) {
    let curr_path = "".to_string();
    let cfg_file = "application.toml";
    let metadata = fs::metadata(cfg_file);
    match metadata {
        Ok(_r) => (cfg_file.to_string(), curr_path),
        Err(_e) => {
            let path = String::from("../bin/");
            let real_file = path.clone() + &cfg_file.to_string();
            info!("configure: {}", real_file);
            let result = fs::metadata(real_file.as_str());
            match result {
                Ok(_r) => (real_file, path),
                Err(_e) => {
                    panic!("can't find application configuration file.");
                }
            }
        }
    }
}

pub fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let cert_file = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(cert_file);
    match rustls::internal::pemfile::certs(&mut reader) {
        Ok(r) => return r,
        Err(e) => {
            error!("Err in load_certs: {:?}", e);
            panic!("");
        }
    }
}

pub fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let rsa_keys = {
        let key_file = fs::File::open(filename).expect("cannot open private key file");
        let mut reader = BufReader::new(key_file);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let key_file = fs::File::open(filename).expect("cannot open private key file");
        let mut reader = BufReader::new(key_file);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    if !pkcs8_keys.is_empty() {
        pkcs8_keys[0].clone()
    } else {
        assert!(!rsa_keys.is_empty());
        rsa_keys[0].clone()
    }
}