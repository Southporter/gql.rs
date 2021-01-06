use config::Config;
use database::Database;

mod config;
mod database;
mod listener;
mod logging;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();

    logging::setup(&config.logging_config).expect("Error setting up logging");

    let database = Database::new(&config);
    listener::listen(database, &config)
}
