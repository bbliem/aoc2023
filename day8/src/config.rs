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

