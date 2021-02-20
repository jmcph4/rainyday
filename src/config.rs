use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};

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

impl TryFrom<File> for Config {
    type Error = String;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let mut contents: String = String::new();
        let mut buf: BufReader<File> = BufReader::new(value);
        match buf.read_to_string(&mut contents) {
            Ok(t) => t,
            Err(e) => return Err(format!("{}", e)),
        };

        match toml::from_str(&contents) {
            Ok(t) => t,
            Err(e) => Err(format!("{}", e)),
        }
    }
}
