pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
