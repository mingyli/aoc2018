use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Error, ErrorKind};

use super::answer::Answer;

struct Claim {
    id: String,
    column: u32,
    row: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn new(id: String, column: u32, row: u32, width: u32, height: u32) -> Claim {
        Claim {
            id,
            column,
            row,
            width,
            height,
        }
    }

    fn values(&self) -> impl Iterator<Item = (u32, u32)> {
        (self.row..(self.row + self.height))
            .cartesian_product(self.column..(self.column + self.width))
    }
}

fn get_claims<R: BufRead>(reader: &mut R) -> Vec<Claim> {
    let pattern = Regex::new(
        r"^#(?P<claim_id>\d+) @ (?P<column>\d+),(?P<row>\d+): (?P<width>\d+)x(?P<height>\d+)$",
    )
    .unwrap();
    let claims = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let cap = pattern.captures(&line).unwrap();
            Claim::new(
                cap["claim_id"].to_string(),
                cap["column"].parse::<u32>().unwrap(),
                cap["row"].parse::<u32>().unwrap(),
                cap["width"].parse::<u32>().unwrap(),
                cap["height"].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<Claim>>();
    claims
}

fn get_counter(claims: &Vec<Claim>) -> HashMap<(u32, u32), u32> {
    let mut counter: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in claims {
        for (r, c) in claim.values() {
            let count = counter.entry((r, c)).or_insert(0);
            *count += 1;
        }
    }
    counter
}

pub fn day3a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let claims = get_claims(&mut reader);
    let counter = get_counter(&claims);
    let answer = counter
        .values()
        .fold(0, |accumulator, count| accumulator + (*count >= 2) as u32);
    Ok(Answer::U(answer))
}

pub fn day3b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let claims = get_claims(&mut reader);
    let counter = get_counter(&claims);
    for claim in claims {
        if claim
            .values()
            .all(|(r, c)| *counter.get(&(r, c)).unwrap() == 1)
        {
            return Ok(Answer::S(claim.id));
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Bad answer."))
}
