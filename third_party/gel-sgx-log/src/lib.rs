#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate log;
extern crate env_logger;



pub fn gel_sgx_log_demo(){

    env_logger::init();
    println!("this is a log demo");

}

