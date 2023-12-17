use std::env;
use std::error::Error;

use day17::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    day17::run(config)?;
    Ok(())
}
