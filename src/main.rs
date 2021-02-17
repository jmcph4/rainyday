use clap::{App, Arg, ArgMatches};

fn main() {
    let _matches: ArgMatches = App::new("rainyday")
        .version("0.0.1")
        .arg(Arg::with_name("input_file").required(true))
        .get_matches();
}
