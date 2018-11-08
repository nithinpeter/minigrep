extern crate minigrep;

use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("error parsing args: {}", error);
        process::exit(-1);
    });

    run(&config).unwrap_or_else(|error| {
        println!("failed to read file: {}", error);
        process::exit(-1);
    });

    println!("{}, {}", config.query, config.fname);
}