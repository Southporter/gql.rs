use crate::config::Config;
use syntax::document::Document;

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
}
