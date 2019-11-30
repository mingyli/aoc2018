use std::fs::File;
use std::io::{self, BufReader};

use crate::answer;
use crate::config;
use crate::day1;
use crate::day2;
use crate::day3;
use crate::day4;
use crate::day5;
use crate::day6;

pub fn run(config: &config::Config) -> io::Result<answer::Answer> {
    let file = File::open(config.filename.clone())?;
    let mut reader = BufReader::new(file);
    let solution = match config.problem.as_ref() {
        "day1a" => day1::day1a,
        "day1b" => day1::day1b,
        "day2a" => day2::day2a,
        "day2b" => day2::day2b,
        "day3a" => day3::day3a,
        "day3b" => day3::day3b,
        "day4a" => day4::day4a,
        "day4b" => day4::day4b,
        "day5a" => day5::day5a,
        "day5b" => day5::day5b,
        "day6a" => day6::day6a,
        "day6b" => day6::day6b,
        _ => day1::day1a,
    };
    solution(&mut reader)
}
