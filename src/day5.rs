use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

use super::answer::Answer;

fn react(chars: &[char]) -> String {
    let mut stack = String::new();
    for &ch in chars {
        if stack.is_empty() {
            stack.push(ch);
        } else {
            let last = stack.chars().last().unwrap();
            if last != ch && last.to_ascii_uppercase() == ch.to_ascii_uppercase() {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
    }
    stack
}

pub fn day5a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let chars = buffer
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    let reacted = react(&chars);
    Ok(Answer::US(reacted.len()))
}

fn filtered_react(removed: char, chars: &[char]) -> String {
    let filtered = chars
        .iter()
        .cloned()
        .filter(|ch| ch.to_ascii_lowercase() != removed)
        .collect::<Vec<char>>();
    react(&filtered)
}

pub fn day5b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let chars = buffer
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    let lexicon: HashSet<char> = HashSet::from_iter(
        chars
            .iter()
            .map(|ch| ch.to_ascii_lowercase())
            .collect::<Vec<char>>()
            .iter()
            .cloned(),
    );
    let best_removal = lexicon
        .iter()
        .min_by_key(|&ch| filtered_react(*ch, &chars).len())
        .unwrap();
    Ok(Answer::US(filtered_react(*best_removal, &chars).len()))
}
