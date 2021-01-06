use crate::config::Config;
use crate::database::Database;
use futures::future;
use log::info;
use net::handlers;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub(crate) fn listen(
    mut database: Database,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Builder::new()
        .threaded_scheduler()
        .core_threads(config.num_threads)
        .thread_name("gql-worker")
        .enable_io()
        .build()
        .expect("Unable to create runtime");

    let mut sockets: Vec<JoinHandle<Result<(), std::io::Error>>> = Vec::new();

    let (db_command, db_receiver) = mpsc::channel::<(String, oneshot::Sender<String>)>(64);
    let _handle = runtime.handle().spawn(async move {
        database.run(db_receiver).await;
    });

    for protocol in &config.protocols {
        info!("setting up protocol: {}", protocol);
        match protocol.as_str() {
            "tcp" => {
                let sender = db_command.clone();
                let handle = runtime.handle();
                let join_handle =
                    handle.spawn(async move { handlers::handle_tcp(9874, sender).await });
                sockets.push(join_handle);
            }
            _ => println!("Protocol not supported: {}", protocol),
        }
    }

    info!("joining");

    runtime.block_on(async {
        let results = future::try_join_all(sockets).await;
        info!("Results from blocking: {:?}", results);
    });
    info!("Ending...");
    runtime.shutdown_timeout(Duration::from_secs(300));
    Ok(())
}
