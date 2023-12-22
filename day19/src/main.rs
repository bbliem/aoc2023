use std::env;
use std::error::Error;

use day19::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    day19::run(config)?;
    Ok(())
}
