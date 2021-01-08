use log::{debug, info};
use tokio;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc::Sender, oneshot};

use crate::connection::Connection;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

type DbSender = Sender<(String, oneshot::Sender<String>)>;

async fn handle_connection(mut conn: Connection<TcpStream>, send: DbSender) -> io::Result<()> {
    loop {
        match conn.read_message().await {
            Ok(Some(content)) => {
                let (send_one, receive_one) = oneshot::channel();
                match send.send((content, send_one)).await.ok() {
                    Some(()) => info!("Sent to database successfully"),
                    None => info!("Send was unsuccessful"),
                };
                match receive_one.await {
                    Ok(response) => {
                        conn.write_message(&response).await?;
                    }
                    Err(e) => info!("Error from db: {}", e),
                };
            }
            Ok(None) => {
                debug!("Message not read");
            }
            Err(_) => break,
        };
    }
    Ok(())
}

pub async fn handle_tcp(port: u32, send: DbSender) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let sender = send.clone();
                tokio::spawn(
                    async move { handle_connection(Connection::new(stream), sender).await },
                );
            }
            Err(e) => {
                info!("Error getting connection: {}", e);
                // break;
            }
        }
    }

    // Ok(())
}
