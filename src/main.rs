use std::path::PathBuf;

use clap::{crate_version, Clap};

pub mod config;

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
