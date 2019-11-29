use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

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

enum Answer {
    I(i32),
    U(u32),
    S(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Answer::I(i) => write!(f, "{}", i),
            Answer::U(u) => write!(f, "{}", u),
            Answer::S(s) => write!(f, "{}", s),
        }
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
        "day2a" => day2a,
        "day2b" => day2b,
        _ => day1a,
    };
    let result = solution(&mut reader)?;
    println!("result: {}", result);

    Ok(())
}

fn day1a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    Ok(Answer::I(
        reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .sum(),
    ))
}

fn day1b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
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
    Ok(Answer::I(current))
}

fn day2a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    fn get_counter(string: &String) -> HashMap<char, u32> {
        let mut counter = HashMap::new();
        for ch in string.chars() {
            let count = counter.entry(ch).or_insert(0);
            *count += 1;
        }
        counter
    }

    let ids = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut twos = 0;
    let mut threes = 0;
    for id in ids {
        let counter = get_counter(&id);
        if counter.values().collect::<HashSet<&u32>>().contains(&2) {
            twos += 1;
        }
        if counter.values().collect::<HashSet<&u32>>().contains(&3) {
            threes += 1;
        }
    }
    Ok(Answer::U(twos * threes))
}

fn day2b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    fn hamming_distance(s1: &String, s2: &String) -> u32 {
        s1.chars()
            .zip(s2.chars())
            .fold(0, |count, (ch1, ch2)| count + (ch1 != ch2) as u32)
    }

    fn common_chars(s1: &String, s2: &String) -> String {
        let mut result = String::new();
        for (ch1, ch2) in s1.chars().zip(s2.chars()) {
            if ch1 == ch2 {
                result.push(ch1);
            }
        }
        result
    }

    let ids = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    for (i, s1) in ids.iter().enumerate() {
        for s2 in ids[i + 1..].iter() {
            let dist = hamming_distance(&s1, &s2);
            if dist == 1 {
                return Ok(Answer::S(common_chars(&s1, &s2)));
            }
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Bad answer."))
}
