pub struct Config {
    pub flag: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, String> {
        let mut flag = String::from("");
        let mut filename = String::from("");

        if args.len() == 1 {
            args.next();
            return Ok(Config { flag, filename });
        } else if args.len() == 2 {
            args.next();
            flag = match args.next() {
                Some(arg) => arg,
                None => return Err(String::from("Something went wrong with the given flag.")),
            };
        } else if args.len() == 3 {
            args.next();
            flag = match args.next() {
                Some(arg) => arg,
                None => return Err(String::from("Something went wrong with the given flag.")),
            };
            filename = match args.next() {
                Some(arg) => arg,
                None => {
                    return Err(String::from(
                        "Something went wrong with the given filename.",
                    ))
                }
            };
        } else {
            let error_message = format!("\nExpected:\n\thuffcomp [OPTIONS] [FILENAME]\n\nFound:\n\t{} arguments were given.\n\nSee 'huffman --help' for more information.\n", args.len());
            return Err(error_message);
        }

        Ok(Config { flag, filename })
    }
}
