extern crate chrono;
extern crate dirs;
extern crate env_logger;
extern crate http_req;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sgx_types;
extern crate sgx_urts;
extern crate toml;

use std::fs;
use std::io::{Read, Write};
use std::path;

use sgx_types::*;
use sgx_urts::SgxEnclave;

mod app_config;
mod ocall_intel;
mod ocall_net;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static ENCLAVE_TOKEN: &'static str = "enclave.token";

extern "C" {
    fn run_server(eid: sgx_enclave_id_t) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // Step 1: try to retrieve the launch token saved by last transaction
    //         if there is no token, then create a new one.
    //
    // try to get the token saved in $HOME */
    let mut home_dir = path::PathBuf::new();
    let use_token = match dirs::home_dir() {
        Some(path) => {
            info!("[+] Home dir is {}", path.display());
            home_dir = path;
            true
        }
        None => {
            error!("[-] Cannot get home dir");
            false
        }
    };

    let token_file: path::PathBuf = home_dir.join(ENCLAVE_TOKEN);
    if use_token == true {
        match fs::File::open(&token_file) {
            Err(_) => {
                info!(
                    "[-] Open token file {} error! Will create one.",
                    token_file.as_path().to_str().unwrap()
                );
            }
            Ok(mut f) => {
                info!("[+] Open token file success! ");
                match f.read(&mut launch_token) {
                    Ok(1024) => {
                        info!("[+] Token file valid!");
                    }
                    _ => warn!("[+] Token file invalid, will create new token file"),
                }
            }
        }
    }

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    let enclave = SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )?;

    // Step 3: save the launch token if it is updated
    if use_token == true && launch_token_updated != 0 {
        // reopen the file with write capablity
        match fs::File::create(&token_file) {
            Ok(mut f) => match f.write_all(&launch_token) {
                Ok(()) => info!("[+] Saved updated launch token!"),
                Err(_) => info!("[-] Failed to save updated launch token!"),
            },
            Err(_) => {
                error!("[-] Failed to save updated enclave token, but doesn't matter");
            }
        }
    }
    Ok(enclave)
}

fn main() {
    app_config::init_env_log();

    let enclave = match init_enclave() {
        Ok(r) => {
            info!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            error!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    info!("[+] Test server in enclave, start!");
    let result = unsafe { run_server(enclave.geteid()) };
    match result {
        sgx_status_t::SGX_SUCCESS => { println!("ECALL success!"); }
        _ => { println!("[-] ECALL Enclave Failed {}!", result.as_str()); }
    }
    enclave.destroy();
}
