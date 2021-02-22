use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::{anyhow, Context};
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
    type Error = anyhow::Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let mut contents: String = String::new();
        let mut buf: BufReader<File> = BufReader::new(value);
        buf.read_to_string(&mut contents)
            .with_context(|| anyhow!("Reading configuration file"))?;

        Ok(toml::from_str(&contents)
            .with_context(|| anyhow!("Parsing configuration file"))?)
    }
}
