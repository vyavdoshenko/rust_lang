pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}