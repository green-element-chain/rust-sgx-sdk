use std::prelude::v1::*;
use std::untrusted::fs;
use std::vec::Vec;
use std::collections::HashMap;
use std::io::{self, Write, Read, BufReader};
use std::net;
use std::net::Shutdown;
use std::sync::Arc;
use rustls::{Session, NoClientAuth, ServerConfig};
use mio::net::{TcpListener, TcpStream};
use Person;

// Token for our listening socket.
const LISTENER: mio::Token = mio::Token(0);

// Which mode the server operates in.
#[derive(Clone)]
enum ServerMode {
    /// Write back received bytes
    Echo,

    /// Do one read, then write a bodged HTTP response and
    /// cleanly close the connection.
    Http,

    /// Forward traffic to/from given port on localhost.
    Forward(u16),
}

/// This binds together a TCP listening socket, some outstanding
/// connections, and a TLS server configuration.
struct TlsServer {
    server: TcpListener,
    connections: HashMap<mio::Token, Connection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
    mode: ServerMode,
}

impl TlsServer {
    fn new(server: TcpListener, mode: ServerMode, cfg: Arc<rustls::ServerConfig>) -> TlsServer {
        TlsServer {
            server,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: cfg,
            mode,
        }
    }

    fn accept(&mut self, poll: &mut mio::Poll) -> bool {
        match self.server.accept() {
            Ok((socket, addr)) => {
                debug!("Accepting new connection from {:?}", addr);

                let tls_session = rustls::ServerSession::new(&self.tls_config);
                let mode = self.mode.clone();

                let token = mio::Token(self.next_id);
                self.next_id += 1;

                self.connections.insert(token, Connection::new(socket, token, mode, tls_session));
                self.connections[&token].register(poll);
                true
            }
            Err(e) => {
                println!("encountered error while accepting connection; err={:?}", e);
                false
            }
        }
    }

    fn conn_event(&mut self, poll: &mut mio::Poll, event: &mio::event::Event) {
        let token = event.token();

        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(poll, event);

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }
}

/// This is a connection which has been accepted by the server,
/// and is currently being served.
///
/// It has a TCP-level stream, a TLS-level session, and some
/// other state/metadata.
struct Connection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    mode: ServerMode,
    tls_session: rustls::ServerSession,
    back: Option<TcpStream>,
    sent_http_response: bool,
}

/// Open a plaintext TCP-level connection for forwarded connections.
fn open_back(mode: &ServerMode) -> Option<TcpStream> {
    match *mode {
        ServerMode::Forward(ref port) => {
            let addr = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), *port);
            let conn = TcpStream::connect(&net::SocketAddr::V4(addr)).unwrap();
            Some(conn)
        }
        _ => None,
    }
}

/// This used to be conveniently exposed by mio: map EWOULDBLOCK
/// errors to something less-errory.
fn try_read(r: io::Result<usize>) -> io::Result<Option<usize>> {
    match r {
        Ok(len) => Ok(Some(len)),
        Err(e) => {
            if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

impl Connection {
    fn new(socket: TcpStream,
           token: mio::Token,
           mode: ServerMode,
           tls_session: rustls::ServerSession)
           -> Connection {
        let back = open_back(&mode);
        Connection {
            socket,
            token,
            closing: false,
            closed: false,
            mode,
            tls_session,
            back,
            sent_http_response: false,
        }
    }

    /// We're a connection, and we have something to do.
    fn ready(&mut self, poll: &mut mio::Poll, ev: &mio::event::Event) {
        // If we're readable: read some TLS.  Then
        // see if that yielded new plaintext.  Then
        // see if the backend is readable too.
        if ev.readiness().is_readable() {
            self.do_tls_read();
            self.try_plain_read();
            self.try_back_read();
        }

        if ev.readiness().is_writable() {
            self.do_tls_write();
        }

        if self.closing && !self.tls_session.wants_write() {
            let _ = self.socket.shutdown(Shutdown::Both);
            self.close_back();
            self.closed = true;
        } else {
            self.reregister(poll);
        }
    }

    /// Close the backend connection for forwarded sessions.
    fn close_back(&mut self) {
        if self.back.is_some() {
            let back = self.back.as_mut().unwrap();
            back.shutdown(Shutdown::Both).unwrap();
        }
        self.back = None;
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
            error!("plaintext read failed: {:?}", rc);
            self.closing = true;
            return;
        }
        let mut persons = Vec::new();

        if !buf.is_empty() {
            let inputstr = std::str::from_utf8(&buf).unwrap();
            debug!("plaintext read {:?}", buf.len());
            self.incoming_plaintext(&buf);

            let result :Person = serde_json::from_str(inputstr).unwrap();
            if result.sendStatus == "end"{
                persons.push(result);
                self.tls_session.write("success\n".as_bytes()).unwrap();
                self.tls_session.send_close_notify();
            }else{
                persons.push(result);
                self.tls_session.write("success\n".as_bytes()).unwrap();
            }

        }else{
            println!("buf is empty");
        }
    }

    fn try_back_read(&mut self) {
        if self.back.is_none() {
            return;
        }

        // Try a non-blocking read.
        let mut buf = [0u8; 1024];
        let back = self.back.as_mut().unwrap();
        let rc = try_read(back.read(&mut buf));

        if rc.is_err() {
            error!("backend read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        let maybe_len = rc.unwrap();

        // If we have a successful but empty read, that's an EOF.
        // Otherwise, we shove the data into the TLS session.
        match maybe_len {
            Some(len) if len == 0 => {
                debug!("back eof");
                self.closing = true;
            }
            Some(len) => {
                self.tls_session.write_all(&buf[..len]).unwrap();
            }
            None => {}
        };
    }

    /// Process some amount of received plaintext.
    fn incoming_plaintext(&mut self, buf: &[u8]) {
        match self.mode {
            ServerMode::Echo => {
                let inputstr = std::str::from_utf8(buf).unwrap();
                println!("Client said: {}", inputstr);
                self.tls_session.write("success\n".as_bytes()).unwrap();
            }
            ServerMode::Http => {
                self.send_http_response_once();
            }
            ServerMode::Forward(_) => {
                self.back.as_mut().unwrap().write_all(buf).unwrap();
            }
        }
    }

    fn send_http_response_once(&mut self) {
        let response = b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from server\r\n";
        if !self.sent_http_response {
            self.tls_session
                .write_all(response)
                .unwrap();
            self.sent_http_response = true;
            self.tls_session.send_close_notify();
            println!("Returned to client successfully!");
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

        if self.back.is_some() {
            poll.register(self.back.as_ref().unwrap(),
                          self.token,
                          mio::Ready::readable(),
                          mio::PollOpt::level() | mio::PollOpt::oneshot())
                .unwrap();
        }
    }

    fn reregister(&self, poll: &mut mio::Poll) {
        poll.reregister(&self.socket,
                        self.token,
                        self.event_set(),
                        mio::PollOpt::level() | mio::PollOpt::oneshot())
            .unwrap();

        if self.back.is_some() {
            poll.reregister(self.back.as_ref().unwrap(),
                            self.token,
                            mio::Ready::readable(),
                            mio::PollOpt::level() | mio::PollOpt::oneshot())
                .unwrap();
        }
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


fn make_config(cert: Vec<rustls::Certificate>, key: rustls::PrivateKey) -> Arc<rustls::ServerConfig> {

    let mut config = rustls::ServerConfig::new(NoClientAuth::new());

    config.set_single_cert_with_ocsp_and_sct(cert, key, vec![], vec![]).unwrap();

    Arc::new(config)
}

pub fn run_mioserver(mio_cert: Vec<rustls::Certificate>, mio_key: rustls::PrivateKey){
    let addr: net::SocketAddr = "0.0.0.0:8443".parse().unwrap();
    let cert = "end.fullchain";
    let key = "end.rsa";
    let mode = ServerMode::Echo;
//    let mode = ServerMode::Http;

    let config = make_config(mio_cert, mio_key);

    let listener = TcpListener::bind(&addr).expect("cannot listen on port");
    let mut poll = mio::Poll::new()
        .unwrap();

    poll.register(&listener,
                  LISTENER,
                  mio::Ready::readable(),
                  mio::PollOpt::level())
        .unwrap();

    let mut tlsserv = TlsServer::new(listener, mode, config);

    println!("\n\n\n\nYou are staring : {}","mio_server");

    let mut events = mio::Events::with_capacity(256);
    'outer: loop {
        poll.poll(&mut events, None)
            .unwrap();

        for event in events.iter() {
            match event.token() {
                LISTENER => {
                    if !tlsserv.accept(&mut poll) {
                        break 'outer;
                    }
                }
                _ => tlsserv.conn_event(&mut poll, &event)
            }
        }
    }

}