use crate::config::Config;
use log::info;
use syntax;
use syntax::document::Document;
use tokio::sync::{mpsc::Receiver, oneshot};

pub(crate) struct Database {
    schema: Document,
    // graph
}

impl Database {
    pub fn new(_config: &Config) -> Self {
        Self {
            schema: Document::default(),
        }
    }

    pub async fn run(&mut self, mut command: Receiver<(String, oneshot::Sender<String>)>) {
        while let Some((gql_str, response)) = command.recv().await {
            // handle connection
            tokio::spawn(async move {
                let parsed = syntax::parse(&gql_str);
                println!("Parsed: {:?}", parsed);
                match response.send("Received input".into()) {
                    Ok(()) => info!("Response sent successfully"),
                    Err(e) => info!("Response from db failed: {}", e),
                };
            });
        }
    }
}
