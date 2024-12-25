mod config;
use std::fs;
use std::env;
use std::process;
use std::error::Error;
use crate::config::Config;

fn main() {
    // rust 中的泛型函数指定类型的方式 (turbofish)
    // func::<type> 与 c++ 中的 func<type> 类似
    // let args_turbofish = env::args().collect::<Vec<String>>();
    // dbg!(args_turbofish);

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);
    Ok(())
}
