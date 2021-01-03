use getopts::Options;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ConfigError {
    ConfigError(String),
    UsageOutputRequested,
    Other(Box<dyn error::Error>)
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ConfigError(s)
                => f.write_fmt(format_args!("Config error: {}", s)),
            ConfigError::UsageOutputRequested
                => f.write_fmt(format_args!("Usage output requested")),
            ConfigError::Other(err)
                => err.fmt(f)
        }
    }
}

impl error::Error for ConfigError {}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn setup_opts() -> Options {
    let mut opts = Options::new();

    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optflag("h", "help", "print usage");

    opts
}

#[derive(Clone,Debug)]
pub struct Config {
    pub input_file_name: String,
    pub output_file_name: String
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result< Config, ConfigError > {
        let opts = setup_opts();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => return Err(ConfigError::Other(e.into()))
        };

        if matches.opt_present("help") {
            print_usage(&args[0], &opts);
            return Err(ConfigError::UsageOutputRequested)
        }

        let output = matches.opt_str("output").unwrap_or(String::from("render.png"));
        let input = match matches.free.first() {
            Some(input) => input.clone(),
            None => return Err(ConfigError::ConfigError(String::from("No input provided")))
        };

        let config = Config {
            input_file_name: input,
            output_file_name: output
        };

        Ok(config)
    }
}
