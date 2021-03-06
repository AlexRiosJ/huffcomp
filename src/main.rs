use std::env;
use std::process;

use huffcomp::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:\n\t{}\n", err);
        process::exit(1);
    });

    if let Err(e) = huffcomp::run(config) {
        eprint!("Application error:\n\t{}\n", e);
        process::exit(2);
    }
}
