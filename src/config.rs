use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pedantic: bool,
    pub respect_private_trackers: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pedantic: false,
            respect_private_trackers: true,
        }
    }
}
