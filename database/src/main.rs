#[macro_use]
extern crate clap;

use clap::App;
use futures::future;
use log::info;
use net::handlers;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::task::JoinHandle;

mod logging;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clap_yaml = load_yaml!("../config/cli.yaml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    let logging_config_file = matches
        .value_of("log_config")
        .unwrap_or("database/config/logging.yaml");

    logging::setup(logging_config_file).expect("Error setting up logging");

    let mut runtime = Builder::new()
        .threaded_scheduler()
        .core_threads(
            matches
                .value_of("threads")
                .unwrap_or("2")
                .parse::<usize>()
                .expect(
                    "Bad Value: Thread command line option must be an integer between 1 and 16",
                ),
        )
        .thread_name("gql-worker")
        .enable_io()
        .build()
        .expect("Unable to create runtime");

    let mut sockets: Vec<JoinHandle<Result<(), std::io::Error>>> = Vec::new();

    for protocol in matches.values_of("protocols").unwrap() {
        info!("setting up protocol: {}", protocol);
        match protocol {
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
        println!("Results from blocking: {:?}", results);
    });
    info!("Ending...");
    runtime.shutdown_timeout(Duration::from_secs(300));
    Ok(())
}
