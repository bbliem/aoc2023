use std::env;
use std::error::Error;

use day13::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    day13::run(config)?;
    Ok(())
}
