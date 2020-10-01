use log::info;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use syntax;

fn handle_database_request(input: &str) -> String {
    let res = syntax::parse(input);
    match res {
        Ok(document) => document.to_string(),
        Err(parse_error) => parse_error.to_string(),
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = String::new();
    info!("Handling connection");
    if let Ok(_num_read) = stream.read_to_string(&mut buffer) {
        info!("read into buffer: {}", buffer);
        let res = handle_database_request(&buffer);
        stream.write_all(&res.into_bytes())
    } else {
        Ok(())
    }
}

pub fn handle_tcp(port: u32) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for incoming in listener.incoming() {
        info!("Got incoming");
        match incoming {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream));
            }
            Err(_) => {}
        }
    }

    Ok(())
}
