use log::info;
use tokio;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;

use crate::connection::Connection;
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
    loop {
        match conn.read_message().await {
            Ok(Some(content)) => {
                let response = handle_database_request(&content);
                conn.write_message(&response).await;
            }
            Ok(None) => continue,
            Err(_) => break,
        };
    }
    Ok(())
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
