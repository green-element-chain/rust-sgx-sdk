use std::clone::Clone;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::string::{String, ToString};
use std::vec::Vec;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerParam {
    pub log_level: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    server_param: Option<Vec<ServerParam>>,
}

impl AppConfig {
    pub fn new(file: &str) -> AppConfig {
        let mut file = File::open(file).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file content");

        toml::from_str(&contents).unwrap()
    }

    fn server_param(&self) -> &ServerParam {
        self.server_param.as_ref().unwrap().get(0).unwrap()
    }

    pub fn log_level(&self) -> String {
        let server_param = self.server_param();
        server_param.log_level.clone().unwrap_or(String::from(""))
    }
}

fn get_app_config() -> String {
    let cfg_file = "application.toml";
    let metadata = fs::metadata(cfg_file);
    match metadata {
        Ok(_r) => cfg_file.to_string(),
        Err(_e) => {
            let path = String::from("../bin/");
            let real_file = path.clone() + &cfg_file.to_string();
            let result = fs::metadata(real_file.as_str());
            match result {
                Ok(_r) => real_file,
                Err(_e) => {
                    panic!("can't find application configuration file.");
                }
            }
        }
    }
}

pub fn init_env_log() {
    let file = get_app_config();
    let app_config = AppConfig::new(file.as_str());
    let level_str = app_config.log_level();

    let level = match level_str.to_ascii_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Off,
    };

    Builder::new()
        .filter(None, level)
        .format(|buf, record| {
            writeln!(buf, "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                record.args()
            )
        })

        .init();
}