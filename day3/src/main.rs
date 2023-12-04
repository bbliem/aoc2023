use std::env;
use std::error::Error;

use day3::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    day3::run(config)?;
    Ok(())
}
