use std::env;
use std::error::Error;

use day1::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    day1::run(config)?;
    Ok(())
}
