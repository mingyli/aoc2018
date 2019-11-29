pub struct Config {
    pub problem: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let problem = args[1].clone();
        let filename = args[2].clone();
        Config { problem, filename }
    }
}
