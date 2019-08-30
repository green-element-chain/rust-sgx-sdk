//! 本模块提供基本的HttpServer服务
//! 所有的业务实现均该模块中实现
use mio::net::TcpListener;
use sgx_types::*;

use config::ApplicationConfig;
use service::config::TlsServerConfig;
use service::tlsserver::TlsServer;
use std::rc::Rc;
use std::sync::Arc;

// 服务相关的模块
pub mod config;
pub mod handler;
pub mod response;
pub mod tlsserver;
// 业务相关的模块
pub mod order;
pub mod project;
pub mod transaction;

// Token for our listening socket.
const LISTENER: mio::Token = mio::Token(0);

pub struct HttpServer {
    config: Rc<ApplicationConfig>,
}

impl HttpServer {
    pub fn new(app_config: Rc<ApplicationConfig>) -> HttpServer {
        HttpServer {
            config: app_config.clone(),
        }
    }

    pub fn start(&self) -> sgx_status_t {
        let port = self.config.server_param().get_port();
        let listen_port = format!("0.0.0.0:{}", port);
        let address = listen_port.parse().unwrap();
        let listener = TcpListener::bind(&address).expect("cannot listen on port");
        info!("listening on http://{}", listen_port);

        let mut tls_config = TlsServerConfig::new(&self.config);

        let mut poll = mio::Poll::new().unwrap();
        poll.register(&listener, LISTENER, mio::Ready::readable(), mio::PollOpt::level()).unwrap();

        let rs_config: Option<rustls::ServerConfig> = tls_config.get();
        if rs_config.is_none() {
            return sgx_status_t::SGX_ERROR_INVALID_SIGNATURE;
        }

        let mut tlsserv = TlsServer::new(listener, Arc::new(rs_config.unwrap()), &self.config);
        let mut events = mio::Events::with_capacity(256);

        let db_max_conn = self.config.server_param().get_db_max_conn();
        'outer: loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        if tlsserv.conn_size() == db_max_conn {
                            continue;
                        }
                        if !tlsserv.accept(&mut poll) {
                            break 'outer;
                        }
                    }
                    _ => tlsserv.conn_event(&mut poll, &event)
                }
            }
        }

        sgx_status_t::SGX_SUCCESS
    }
}