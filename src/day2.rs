use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use crate::answer::Answer;

pub fn day2a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
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

pub fn day2b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
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
    let (s1, s2) = ids
        .iter()
        .tuple_combinations::<(_, _)>()
        .find(|(s1, s2)| hamming_distance(&s1, &s2) == 1)
        .unwrap();
    return Ok(Answer::S(common_chars(&s1, &s2)));
}
