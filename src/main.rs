use std::env;
use std::fs::File;
use std::io::{self, BufReader};

mod answer;
mod day1;
mod day2;
mod day3;

struct Config {
    problem: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let problem = args[1].clone();
        let filename = args[2].clone();
        Config { problem, filename }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let file = File::open(config.filename)?;
    let mut reader = BufReader::new(file);

    let solution = match config.problem.as_ref() {
        "day1a" => day1::day1a,
        "day1b" => day1::day1b,
        "day2a" => day2::day2a,
        "day2b" => day2::day2b,
        "day3a" => day3::day3a,
        "day3b" => day3::day3b,
        _ => day1::day1a,
    };
    let result = solution(&mut reader)?;
    println!("result: {}", result);

    Ok(())
}
