use clap::{App, Arg, ArgMatches, crate_version};

fn main() {
    let _matches: ArgMatches = App::new("rainyday")
        .version(crate_version!())
        .arg(Arg::with_name("input_file").required(true))
        .get_matches();
}
