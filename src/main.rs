use std::path::PathBuf;

use clap::{crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!())]
#[allow(dead_code)]
pub struct Opts {
    input_file: PathBuf,
}

fn main() {
    let _opts: Opts = Opts::parse();
}
