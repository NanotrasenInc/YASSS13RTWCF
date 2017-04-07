use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::io::{Read, Write};
use std::io;
use slog::Logger;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::iter;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    #[doc(hidden)]
    pub static ref LOGGER: Logger = {
        use logs::LOGGER as HEAD_LOGGER;
        HEAD_LOGGER.new(None)
    };

    pub static ref CLIENTS: RwLock<HashMap<SocketAddr, Arc<RwLock<Client>>>> = {
        RwLock::new(HashMap::new())
    };
}


pub fn run(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    // TODO: Set listener to non-blocking and have better control here.
    for connection in listener.incoming().map(|x| x.expect("TODO: handle IO errors.")) {
        let address = format!("{:?}", connection.peer_addr().unwrap());
        match spawn_client(connection) {
            // TODO: handle server polling with better logging, or maybe at all!
            Ok(_) => {
                debug!(LOGGER, "Accepted new client."; "address" => address);
            }
            Err(e) => {
                error!(LOGGER, "Error while accepting new client";
                    "address" => address, "error" => format!("{:?}", e));
            }
        }
    }
}

fn spawn_client(client: TcpStream) -> io::Result<thread::Thread> {
    let name = format!("client_{}", client.peer_addr()?);
    let handle = thread::Builder::new()
        .name(name)
        .spawn(move || {
            let address = format!("{:?}", client.peer_addr().unwrap());
            match do_client(client) {
                Ok(()) => debug!(LOGGER, "Dropping client"; "client" => address),
                Err(e) => error!(LOGGER, "Error with client connection.";
                    "client" => address, "error" => format!("{:?}", e))
            }
        })?;
    Ok(handle.thread().clone())
}

fn do_client(mut connection: TcpStream) -> io::Result<()> {
    let client = match handshake(&mut connection) {
        Ok(client) => {
            connection.write_u8(1)?;
            client
        }
        Err(_) => {
            return connection.write_u8(0);
        }
    }



    Ok(())
}

fn handshake(connection: &mut TcpStream) -> io::Result<Arc<RwLock<Client>>> {
    let mut buf = vec![0, 0];
    connection.read_exact(&mut buf)?;
    match buf.as_slice() {
        b"NT" => {},
        b"SY" => return Err(io_error("Server polling is unimplemented.")),
        _ => return Err(io_error("ID bytes incorrect."))
    };

    let len = connection.read_u8()?;
    buf = empty_buf(len as usize);
    connection.read_exact(&mut buf)?;
    if buf.as_slice() != env!("CARGO_PKG_VERSION").as_bytes() {
        return Err(io_error("Version mismatch."));
    }

    let len = connection.read_u32::<NetworkEndian>()? as usize;
    buf = empty_buf(len);
    connection.read_exact(&mut buf)?;
    let nick = match String::from_utf8(buf) {
        Ok(nick) => nick,
        Err(_) => return Err(io_error("Nick is not valid UTF-8"))
    };

    let client_object = Arc::new(RwLock::new(Client {
        nick: nick,
        thread: thread::current(),
        address: connection.peer_addr().unwrap()
    }));



    let mut clients = CLIENTS.write().unwrap();
    clients.insert(connection.peer_addr().unwrap(), client_object.clone());

    Ok(client_object)
}

/// Create an empty buffer of a certain length for usage by `Read::read_exact()`.
#[inline(always)]
fn empty_buf(len: usize) -> Vec<u8> {
    // There has to be a better approach to this.
    iter::repeat(0).take(len).collect::<Vec<u8>>()
}

#[inline(always)]
fn io_error<P: AsRef<str>>(msg: P) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg.as_ref())
}

#[derive(Debug)]
pub struct Client {
    nick: String,
    thread: thread::Thread,
    address: SocketAddr
}
