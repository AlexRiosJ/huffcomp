pub struct Config {
    pub flag: String,
    pub filename: String,
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
