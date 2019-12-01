use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use super::answer::Answer;

type Vertex = char;
type Edge = (Vertex, Vertex);
type Graph = HashMap<Vertex, HashSet<Vertex>>;

fn get_edges<R: BufRead>(reader: &mut R) -> Vec<Edge> {
    let pattern =
        Regex::new(r"^Step (?P<from>.) must be finished before step (?P<to>.) can begin.$")
            .unwrap();
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let captures = pattern.captures(&line).unwrap();
            let from: Vertex = captures["from"].chars().next().unwrap();
            let to: Vertex = captures["to"].chars().next().unwrap();
            (from, to)
        })
        .collect()
}

fn get_graph(edges: &[Edge]) -> Graph {
    let mut graph: Graph = Graph::new();
    for &(from, to) in edges {
        graph.entry(to).or_default();
        let outgoing_edges: &mut HashSet<Vertex> = graph.entry(from).or_default();
        outgoing_edges.insert(to);
    }
    graph
}

fn get_indegrees(graph: &Graph) -> HashMap<Vertex, u32> {
    let mut indegrees: HashMap<Vertex, u32> = HashMap::new();
    for (&vertex, neighbors) in graph {
        indegrees.entry(vertex).or_insert(0);
        for &neighbor in neighbors {
            let count = indegrees.entry(neighbor).or_insert(0);
            *count += 1;
        }
    }
    indegrees
}

fn topological_sort(graph: &Graph) -> Vec<Vertex> {
    let mut indegrees = get_indegrees(&graph);
    let mut sequence: Vec<Vertex> = Vec::new();
    fn get_candidates(indegrees: &HashMap<Vertex, u32>, sequence: &[Vertex]) -> Vec<Vertex> {
        indegrees
            .iter()
            .filter(|(_v, &indegree)| indegree == 0)
            .map(|(&v, _indegree)| v)
            .filter(|v| !sequence.contains(v))
            .collect::<Vec<Vertex>>()
    };
    let mut candidates: Vec<Vertex> = get_candidates(&indegrees, &sequence);
    while !candidates.is_empty() {
        candidates.sort();
        let &start: &Vertex = candidates.first().unwrap();
        sequence.push(start);
        let neighbors = &graph[&start];
        for neighbor in neighbors {
            *indegrees.get_mut(&neighbor).unwrap() -= 1;
        }
        candidates = get_candidates(&indegrees, &sequence);
    }
    sequence
}

pub fn day7a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let edges = get_edges(&mut reader);
    let graph = get_graph(&edges);
    let sequence = topological_sort(&graph);
    Ok(Answer::S(sequence.iter().collect()))
}

fn get_duration(vertex: Vertex) -> u32 {
    60 + vertex as u32 - 'A' as u32 + 1
}

pub fn day7b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    const NUM_WORKERS: usize = 5;
    let edges = get_edges(&mut reader);
    let graph = get_graph(&edges);
    let mut indegrees = get_indegrees(&graph);
    let mut jobs: HashMap<Vertex, u32> = HashMap::new();
    let mut done: Vec<Vertex> = Vec::new();
    fn get_candidates(
        indegrees: &HashMap<Vertex, u32>,
        jobs: &HashMap<Vertex, u32>,
        done: &[Vertex],
    ) -> Vec<Vertex> {
        indegrees
            .iter()
            .filter(|(_v, &indegree)| indegree == 0)
            .map(|(&v, _indegree)| v)
            .filter(|v| !jobs.contains_key(v))
            .filter(|v| !done.contains(v))
            .collect::<Vec<Vertex>>()
    }
    let mut t = 0;
    while done.len() < graph.len() {
        let mut candidates = get_candidates(&indegrees, &jobs, &done);
        candidates.sort();
        for candidate in candidates {
            if jobs.len() < NUM_WORKERS {
                jobs.insert(candidate, get_duration(candidate));
            }
        }
        for (v, duration) in jobs.iter_mut() {
            *duration -= 1;
            if *duration == 0 {
                done.push(*v);
                let neighbors = &graph[&v];
                for neighbor in neighbors {
                    *indegrees.get_mut(&neighbor).unwrap() -= 1;
                }
            }
        }
        jobs.retain(|_v, &mut duration| duration > 0);
        t += 1;
    }
    Ok(Answer::U(t))
}
