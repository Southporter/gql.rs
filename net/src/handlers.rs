use std::io;
use std::net::{TcpListener, TcpStream};

fn handle_connection(_stream: TcpStream) {}

pub fn handle_tcp(port: u32) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}
