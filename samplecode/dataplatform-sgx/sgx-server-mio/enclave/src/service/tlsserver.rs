#[allow(dead_code)]
use mio::net::{TcpListener, TcpStream};
use rustls::Session;

use config::ApplicationConfig;
use service::handler::HttpHandler;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::Shutdown;
use std::rc::Rc;
use std::str;
use std::sync::Arc;
use std::vec::Vec;

/// This binds together a TCP listening socket, some outstanding
/// connections, and a TLS server configuration.
pub struct TlsServer {
    server: TcpListener,
    connections: HashMap<mio::Token, Connection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
    http_handle: Arc<HttpHandler>,
}

impl TlsServer {
    pub fn new(server: TcpListener, cfg: Arc<rustls::ServerConfig>, app_config: &Rc<ApplicationConfig>) -> TlsServer {
        TlsServer {
            server,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: cfg,
            http_handle: Arc::new(HttpHandler::new(app_config)),
        }
    }

    pub fn conn_size(&mut self) -> u16 {
        self.connections.len() as u16
    }

    pub fn accept(&mut self, poll: &mut mio::Poll) -> bool {
        match self.server.accept() {
            Ok((socket, addr)) => {
                if self.connections.len() > 40 {
                    let _ = socket.shutdown(Shutdown::Both);
                    return true;
                }

                debug!("Accepting new connection from {:?}", addr);
                let tls_session = rustls::ServerSession::new(&self.tls_config);

                let token = mio::Token(self.next_id);
                self.next_id += 1;

                self.connections.insert(token, Connection::new(socket, token, tls_session, &self.http_handle));
                self.connections[&token].register(poll);
                true
            }
            Err(e) => {
                error!("encountered error while accepting connection; err={:?}", e);
                false
            }
        }
    }

    pub fn conn_event(&mut self, poll: &mut mio::Poll, event: &mio::event::Event) {
        let token = event.token();
        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(poll, event);

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
            debug!("number of connections is: {}", self.connections.len());
        }
    }
}

/// It has a TCP-level stream, a TLS-level session, and some
/// other state/metadata.
struct Connection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    tls_session: rustls::ServerSession,
    sent_http_response: bool,
    http_handler: Arc<HttpHandler>,
}

impl Connection {
    fn new(socket: TcpStream, token: mio::Token, tls_session: rustls::ServerSession, handler: &Arc<HttpHandler>) -> Connection {
        Connection {
            socket,
            token,
            closing: false,
            closed: false,
            tls_session,
            sent_http_response: false,
            http_handler: handler.clone(),
        }
    }

    /// We're a connection, and we have something to do.
    pub fn ready(&mut self, poll: &mut mio::Poll, ev: &mio::event::Event) {
        // If we're readable: read some TLS. Then
        // see if that yielded new plaintext. Then
        if ev.readiness().is_readable() {
            self.do_tls_read();
            self.try_plain_read();
        }

        if ev.readiness().is_writable() {
            self.do_tls_write();
        }

        if self.closing {
            let _ = self.socket.shutdown(Shutdown::Both);
            self.closed = true;
        } else {
            self.reregister(poll);
        }
    }

    fn do_tls_read(&mut self) {
        // Read some TLS data.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            let err = rc.unwrap_err();
            if let io::ErrorKind::WouldBlock = err.kind() {
                return;
            }

            error!("read error {:?}", err);
            self.closing = true;
            return;
        }

        if rc.unwrap() == 0 {
            debug!("eof");
            self.closing = true;
            return;
        }

        // Process newly-received TLS messages.
        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            error!("cannot process packet: {:?}", processed);
            self.closing = true;
            return;
        }
    }

    fn try_plain_read(&mut self) {
        // Read and process all available plaintext.
        let mut buf = Vec::new();

        let rc = self.tls_session.read_to_end(&mut buf);
        if rc.is_err() {
            debug!("plaintext read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        if !buf.is_empty() {
            let buf_str = str::from_utf8(&buf).unwrap();
            let resp = self.http_handler.process_request(&buf_str);
            self.send_http_response_once(resp.as_str());
        }
    }

    fn send_http_response_once(&mut self, response: &str) {
        let response = format!("{}", response);
        if !self.sent_http_response {
            self.tls_session.write_all(response.as_bytes()).unwrap();
            self.sent_http_response = true;
            self.tls_session.send_close_notify();
            debug!("Returned to client successfully!");
        }
    }

    fn do_tls_write(&mut self) {
        let rc = self.tls_session.write_tls(&mut self.socket);
        if rc.is_err() {
            error!("write failed {:?}", rc);
            self.closing = true;
            return;
        }
    }

    fn register(&self, poll: &mut mio::Poll) {
        poll.register(&self.socket,
            self.token,
            self.event_set(),
            mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

    fn reregister(&self, poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
            self.token,
            self.event_set(),
            mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();
    }

    /// What IO events we're currently waiting for,
    /// based on wants_read/wants_write.
    fn event_set(&self) -> mio::Ready {
        let rd = self.tls_session.wants_read();
        let wr = self.tls_session.wants_write();

        if rd && wr {
            mio::Ready::readable() | mio::Ready::writable()
        } else if wr {
            mio::Ready::writable()
        } else {
            mio::Ready::readable()
        }
    }

    fn is_closed(&self) -> bool {
        self.closed
    }
}
