use std::ffi::CStr;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::io::IntoRawFd;

use http_req::uri::Uri;
use sgx_types::*;

pub fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }
    unreachable!("Cannot lookup address");
}

#[no_mangle]
pub extern "C"
fn ocall_get_url_socket(req_url: *const c_char, ret_fd: *mut c_int) -> sgx_status_t {
    //let url = "https://tieba.baidu.com/";
    let c_str: &CStr = unsafe { CStr::from_ptr(req_url) };
    let url: &str = c_str.to_str().unwrap();
    let req_uri: Uri = url.parse().unwrap();

    let port = req_uri.corr_port();
    let host = req_uri.host().unwrap_or("");

    let socket_addr = lookup_ipv4(host, port);
    let tcp_stream = TcpStream::connect(&socket_addr)
        .expect("failed to create connect tls stream.");

    unsafe { *ret_fd = tcp_stream.into_raw_fd(); }

    sgx_status_t::SGX_SUCCESS
}