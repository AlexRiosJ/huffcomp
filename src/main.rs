use std::env;
use std::process;

use huffcomp::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:\n\t{}", err);
        process::exit(1);
    });

    if let Err(e) = huffcomp::run(config) {
        eprint!("Application error:\n\t{}", e);
        process::exit(1);
    }
}
