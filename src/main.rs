use std::path::PathBuf;

use clap::{crate_version, Clap};
#[macro_use]
extern crate enum_display_derive;

pub mod config;
pub mod protocol;

#[derive(Clap)]
#[clap(version = crate_version!())]
#[allow(dead_code)]
pub struct Opts {
    input_file: PathBuf,
    #[clap(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let _opts: Opts = Opts::parse();
}
