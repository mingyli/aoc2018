use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
        "day1a" => day1a,
        "day1b" => day1b,
        _ => day1a,
    };
    let result = solution(&mut reader)?;
    println!("result: {}", result);

    Ok(())
}

fn day1a<R: BufRead>(reader: &mut R) -> io::Result<i32> {
    Ok(reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .sum())
}

fn day1b<R: BufRead>(reader: &mut R) -> io::Result<i32> {
    let diffs_stream = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .cycle();
    let mut prefix_sums = diffs_stream.scan(0, |sum, diff| {
        *sum = *sum + diff;
        Some(*sum)
    });
    let mut visited: HashSet<i32> = HashSet::new();
    let mut current = prefix_sums.next().unwrap();
    while !visited.contains(&current) {
        visited.insert(current);
        current = prefix_sums.next().unwrap();
    }
    Ok(current)
}
