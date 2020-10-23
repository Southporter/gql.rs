use log::info;
use tokio;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;

use crate::connection::Connection;
use crate::message::Message;
use syntax;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn handle_database_request(input: &str) -> String {
    let res = syntax::parse(input);
    match res {
        Ok(document) => document.to_string(),
        Err(parse_error) => parse_error.to_string(),
    }
}

async fn handle_connection(mut conn: Connection<TcpStream>) -> io::Result<()> {
    let _message: Message = conn.read_message().await.unwrap().unwrap();
    Ok(())
    // let mut buffer: Vec<u8> = Vec::new();
    // info!("Handling connection");
    // if let Ok(num_read) = stream.read(buffer.as_mut_slice()).await? {
    //     info!("read into buffer: {}; {}", buffer, num_read);
    //     let res = handle_database_request();
    //     info!("Result from database: {:?}", res);
    //     stream.write_all(&res.into_bytes()).await
    // } else {
    //     info!("Error reading to string");
    //     Ok(())
    // }
}

pub async fn handle_tcp(port: u32) -> io::Result<()> {
    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    while let Some(conn) = listener.next().await {
        info!("Got incoming");
        match conn {
            Ok(stream) => {
                tokio::spawn(async move { handle_connection(Connection::new(stream)).await });
            }
            Err(_) => {}
        }
    }

    Ok(())
}
