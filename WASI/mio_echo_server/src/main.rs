#![allow(unused_imports)]
#![cfg(any(unix, target_os = "wasi"))]
use console::style;
use console::Style;
use console::Term;
use log::{debug, error, info, trace, warn};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::FromRawFd;
use std::str::from_utf8;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);
fn main() -> anyhow::Result<()> {
    let term = Term::stdout();
    term.write_line("Starting the app!")?;
    //setup_logger().unwrap(); // BREAKS PROGRAMM
    //trace!("Starting the app!");

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(512);

    // Setup the TCP server socket.
    println!("events capacity: {}", events.capacity());
    let mut server = {
        let stdlistener = unsafe { std::net::TcpListener::from_raw_fd(3) };
        stdlistener.set_nonblocking(true).unwrap();
        term.write_line("uning preopened fd for server socket!")?;
        TcpListener::from_std(stdlistener)
    };
    let max_sockets = 25_usize;
    term.write_line(
        format!(
            "Setted poll capacity {}, max connections: {}",
            events.capacity(),
            max_sockets
        )
        .as_str(),
    )?;
    let mut next_socket_i = 0_usize;
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;
    let mut connections = HashMap::new();
    let mut unique_token = Token(SERVER.0 + 1);
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                //accept connection
                SERVER => loop {
                    let (mut connection, _address) = match server.accept() {
                        Ok((connection, _address)) => {
                            if next_socket_i == max_sockets {
                                return Ok(());
                            }
                            next_socket_i += 1;
                            (connection, _address)
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    };
                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;
                    connections.insert(token, connection);
                },
                token => {
                    term.write_line("got connection!")?;
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(poll.registry(), connection, event)?
                    } else {
                        false
                    };
                    if done {
                        next_socket_i -= 1;
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
const DATA: &[u8] = b"Hello world!\n";

/// Returns `true` if the connection is done.
fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    if event.is_writable() {
        match connection.write(DATA) {
            // We want to write the entire `DATA` buffer in a single go. If we
            Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => registry.reregister(connection, event.token(), Interest::READABLE)?,
            Err(ref err) if would_block(err) => {}
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, connection, event)
            }
            Err(err) => return Err(err),
        }
    }

    if event.is_readable() {
        let mut connection_closed = false;

        //let mut received_data = SmallVec::<[u8; 4096]>::new();
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
