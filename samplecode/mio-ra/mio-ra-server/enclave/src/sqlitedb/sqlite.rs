use std::io::{self, BufReader, Read, Write};
use std::slice;
use std::prelude::v1::*;
use std::vec::Vec;
use sgx_types::*;

use crate::sqlitedb::opening;
use crate::sqlitedb::typical;


pub fn start_db(existed:uint8_t) {

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    println!("{}", &rust_raw_string);


    //sqlite test
    {
        println!("sqlite opening test:");
        opening::opening(existed);

        println!("sqlite typical test:");
        typical::typical();
    }
}
