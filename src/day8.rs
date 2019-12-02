use std::io::{self, BufRead};

use super::answer::Answer;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new() -> Node {
        Node {
            children: vec![],
            metadata: vec![],
        }
    }
}

fn get_tree(iter: &mut dyn Iterator<Item = u32>) -> Node {
    let mut root = Node::new();
    let num_children = iter.next().unwrap();
    let num_metadata = iter.next().unwrap();
    for _ in 0..num_children {
        let child = get_tree(iter);
        root.children.push(child);
    }
    for _ in 0..num_metadata {
        let metadata = iter.next().unwrap();
        root.metadata.push(metadata);
    }
    root
}

fn sum_metadata(root: &Node) -> u32 {
    root.metadata.iter().sum::<u32>() + root.children.iter().map(sum_metadata).sum::<u32>()
}

pub fn day8a<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let mut iter = buffer
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap());
    let root = get_tree(&mut iter);
    Ok(Answer::U(sum_metadata(&root)))
}

fn get_value(root: &Node) -> u32 {
    if root.children.is_empty() {
        root.metadata.iter().sum()
    } else {
        root.metadata
            .iter()
            .map(|index| *index as usize)
            .filter_map(|index| root.children.get(index - 1))
            .map(get_value)
            .sum()
    }
}

pub fn day8b<R: BufRead>(reader: &mut R) -> io::Result<Answer> {
    let mut buffer = String::new();
    let _num_bytes = reader.read_line(&mut buffer).unwrap();
    let mut iter = buffer
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap());
    let root = get_tree(&mut iter);
    Ok(Answer::U(get_value(&root)))
}
