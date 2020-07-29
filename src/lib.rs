use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    flag: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let flag = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a configuration flag."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name."),
        };

        Ok(Config { flag, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.filename);
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())
}
