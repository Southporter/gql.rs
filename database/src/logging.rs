use log4rs::{self, Error};
use std::default::Default;

pub fn setup(config_file_path: &str) -> Result<(), Error> {
    log4rs::init_file(config_file_path, Default::default())
}
