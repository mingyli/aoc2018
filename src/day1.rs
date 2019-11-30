use std::collections::HashSet;
use std::io::{self, BufRead};

use super::answer::Answer;

pub fn day1a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    Ok(Answer::I(
        reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .sum(),
    ))
}

pub fn day1b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    let diffs_stream = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .cycle();
    let mut prefix_sums = diffs_stream.scan(0, |sum, diff| {
        *sum += diff;
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
