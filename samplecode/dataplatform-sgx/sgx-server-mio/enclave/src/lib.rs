#![crate_name = "miora"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate base64;
extern crate bit_vec;
extern crate chrono;
extern crate env_logger_gel as env_logger;
extern crate httparse;
extern crate itertools;
#[macro_use]
extern crate log_gel as log;
extern crate mio;
extern crate num_bigint;
extern crate rusthex;
extern crate rustls;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sgx_rand;
extern crate sgx_tcrypto;
extern crate sgx_tse;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;
extern crate sgx_untrusted_time as time;
extern crate sqlite3;
extern crate toml;
extern crate webpki;
extern crate webpki_roots;
extern crate yasna;

use sgx_types::*;

use config::ApplicationConfig;
use service::HttpServer;
use std::rc::Rc;
use utils::file::*;

pub mod client;
pub mod config;
pub mod logger;
pub mod service;
pub mod utils;

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn run_server(socket_fd: c_int) -> sgx_status_t {
    let (file, path) = get_application_config();
    let app_config = ApplicationConfig::new(file.as_str(), path);
    logger::init_env_log(app_config.server_param().log_level());
    //debug!("{:?}", app_config);

    let server = HttpServer::new(Rc::new(app_config));
    return server.start();
}