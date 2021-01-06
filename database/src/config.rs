use clap::{load_yaml, App};

pub struct Config {
    pub num_threads: usize,
    pub logging_config: String,
    pub protocols: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let clap_yaml = load_yaml!("../config/cli.yaml");
        let matches = App::from_yaml(clap_yaml).get_matches();
        let num_threads = matches
            .value_of("threads")
            .unwrap_or("2")
            .parse::<usize>()
            .expect("Bad Value: Thread command line option must be an integer between 1 and 16");

        let logging_config = matches
            .value_of("log_config")
            .unwrap_or("database/config/logging.yaml");
        let protocols = matches
            .value_of("protocols")
            .expect("No protocols where provided");

        Self {
            num_threads,
            logging_config: String::from(logging_config),
            protocols: protocols.split(",").map(|s| s.into()).collect(),
        }
    }
}
