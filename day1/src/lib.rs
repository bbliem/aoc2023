use std::error::Error;
use std::fs;

mod part1;
mod part2;

pub struct Config {
    pub file_path1: String,
    pub file_path2: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            file_path1: args[1].clone(),
            file_path2: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Part 1: Reading file {}", config.file_path1);
    let contents = fs::read_to_string(config.file_path1)?;
    part1::run(&contents)?;

    println!("Part 2: Reading file {}", config.file_path2);
    let contents = fs::read_to_string(config.file_path2)?;
    part2::run(&contents)?;

    Ok(())
}
