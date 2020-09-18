#[macro_use]
extern crate clap;

use clap::App;
use net::handlers;
use std::thread;

fn main() {
    let clap_yaml = load_yaml!("../config/cli.yaml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    // let config_file = matches
    //     .value_of("config")
    //     .unwrap_or("../config/default.yaml");

    for protocol in matches.values_of("protocols").unwrap() {
        println!("setting up protocol: {}", protocol);
        match protocol {
            "tcp" => {
                let _ = thread::Builder::new()
                    .name("tcp_handler".to_string())
                    .spawn(move || handlers::handle_tcp(9876));
            }
            _ => println!("Protocol not supported: {}", protocol),
        }
    }
}
