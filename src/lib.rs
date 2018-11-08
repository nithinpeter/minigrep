use std::error;
use std::fs;

pub struct Config {
    pub fname: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Result::Err("less number of args than required")
        } else {
            Result::Ok(Config {
                fname: args[1].clone(),
                query: args[2].clone(),
            })
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let _contents = fs::read_to_string(&config.fname)?;
    Ok(())
}