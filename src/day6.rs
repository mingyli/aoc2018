use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

use super::answer::Answer;

#[derive(Clone, Copy, Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
}

type PointID = usize;

impl Point {
    fn dist(self, other: Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn get_points<R: BufRead>(reader: &mut R) -> HashMap<PointID, Point> {
    let pattern = Regex::new(r"^(?P<x>\d+), (?P<y>\d+)$").unwrap();
    let points: HashMap<PointID, Point> = reader
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .map(|(id, line)| {
            let caps = pattern.captures(&line).unwrap();
            let x = caps["x"].parse::<i32>().unwrap();
            let y = caps["y"].parse::<i32>().unwrap();
            (id, Point { x, y })
        })
        .collect::<HashMap<PointID, Point>>();
    points
}

pub fn day6a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let points = get_points(&mut reader);
    let left = points.values().map(|p| p.x).min().unwrap();
    let right = points.values().map(|p| p.x).max().unwrap();
    let top = points.values().map(|p| p.y).min().unwrap();
    let bottom = points.values().map(|p| p.y).max().unwrap();
    let closest: HashMap<Point, Option<PointID>> = (left..=right)
        .cartesian_product(top..=bottom)
        .map(|(x, y)| {
            let dists: HashMap<(Point, PointID), u32> = points
                .iter()
                .map(|(&id, &p)| ((p, id), p.dist(Point { x, y })))
                .collect();
            let ((_closest_point, closest_id), min_dist) =
                dists.iter().min_by_key(|(_p, &dist)| dist).unwrap();
            if dists.values().filter(|&d| d == min_dist).count() == 1 {
                (Point { x, y }, Some(*closest_id))
            } else {
                (Point { x, y }, None)
            }
        })
        .collect();
    // Can exclude a PointID if there exists a boundary point that it is closest to.
    let can_exclude: HashSet<PointID> = points
        .keys()
        .filter(|&&id| {
            for x in left..=right {
                let p = Point { x, y: top };
                if closest[&p].is_some() && closest[&p].unwrap() == id {
                    return true;
                }
                let q = Point { x, y: bottom };
                if closest[&q].is_some() && closest[&q].unwrap() == id {
                    return true;
                }
            }
            for y in top..=bottom {
                let p = Point { x: left, y };
                if closest[&p].is_some() && closest[&p].unwrap() == id {
                    return true;
                }
                let q = Point { x: right, y };
                if closest[&q].is_some() && closest[&q].unwrap() == id {
                    return true;
                }
            }
            false
        })
        .cloned()
        .collect();
    let counter: HashMap<PointID, usize> = points
        .keys()
        .filter(|&id| !can_exclude.contains(id))
        .map(|&id: &PointID| {
            let count = closest
                .values()
                .filter(|opt_id: &&Option<PointID>| opt_id.is_some() && opt_id.unwrap() == id)
                .count();
            (id, count)
        })
        .collect();
    Ok(Answer::US(*counter.values().max().unwrap()))
}

pub fn day6b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    const THRESHOLD: u32 = 10000;
    let points = get_points(&mut reader);
    let left = points.values().map(|p| p.x).min().unwrap();
    let right = points.values().map(|p| p.x).max().unwrap();
    let top = points.values().map(|p| p.y).min().unwrap();
    let bottom = points.values().map(|p| p.y).max().unwrap();
    let weights: HashMap<Point, u32> = (left..=right)
        .cartesian_product(top..=bottom)
        .map(|(x, y)| {
            let point = Point { x, y };
            (point, points.values().map(|p| p.dist(point)).sum())
        })
        .collect();
    Ok(Answer::US(
        weights.values().filter(|&&w| w < THRESHOLD).count(),
    ))
}
