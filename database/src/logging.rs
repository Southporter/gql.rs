use syslog::{Facility, Forma};
use log;

pub fn setup(config_file_path: &str) -> Result<()> {
    log4rs::init_file(config_file_path)
}
