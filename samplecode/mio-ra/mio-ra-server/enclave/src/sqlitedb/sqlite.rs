use sgx_types::*;
use std::io::{self, BufReader, Read, Write};
use std::prelude::v1::*;
use std::slice;
use std::vec::Vec;

use crate::sqlitedb::opening;

pub fn start_db(existed: uint8_t) {
    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    println!("{}", &rust_raw_string);

    //sqlite test
    {
        println!("sqlite opening test:");
        opening::base_test(existed);
    }
}
