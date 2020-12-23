use crate::config::Config;
use crate::database::Database;
use futures::future;
use log::info;
use net::handlers;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::task::JoinHandle;

pub(crate) fn listen(
    _database: Database,
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

    for protocol in &config.protocols {
        info!("setting up protocol: {}", protocol);
        match protocol.as_str() {
            "tcp" => {
                let handle = runtime.handle();
                let join_handle = handle.spawn(async move { handlers::handle_tcp(9874).await });
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
