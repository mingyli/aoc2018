extern crate chrono;
use chrono::{Duration, NaiveDateTime, Timelike};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::mem;

use super::answer::Answer;

#[derive(Debug)]
struct Record {
    datetime: NaiveDateTime,
    description: String,
}

#[derive(Debug)]
enum Action {
    BeginShift,
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Event {
    datetime: NaiveDateTime,
    action: Action,
    guard: u32,
}

fn get_records<R: BufRead>(mut reader: &mut R) -> Vec<Record> {
    let pattern = Regex::new(r"^\[(?P<datetime>.+)\] (?P<description>.+)$").unwrap();
    let mut records = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let cap = pattern.captures(&line).unwrap();
            let datetime =
                NaiveDateTime::parse_from_str(&cap["datetime"], "%Y-%m-%d %H:%M").unwrap();
            let description = cap["description"].to_string();
            Record {
                datetime,
                description,
            }
        })
        .collect::<Vec<Record>>();
    records.sort_by_key(|record| record.datetime);
    records
}

fn get_events(records: &Vec<Record>) -> Vec<Event> {
    fn get_guard_id(record: &Record) -> u32 {
        let pattern = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
        let cap = pattern.captures(&record.description).unwrap();
        let id = cap["id"].parse::<u32>().unwrap();
        id
    }

    let mut events: Vec<Event> = Vec::new();
    let mut current_guard = get_guard_id(&records[0]);
    for record in records {
        let event = match record.description.as_str() {
            "falls asleep" => Event {
                datetime: record.datetime,
                action: Action::FallAsleep,
                guard: current_guard,
            },
            "wakes up" => Event {
                datetime: record.datetime,
                action: Action::WakeUp,
                guard: current_guard,
            },
            _ => {
                current_guard = get_guard_id(&record);
                Event {
                    datetime: record.datetime,
                    action: Action::BeginShift,
                    guard: current_guard,
                }
            }
        };
        events.push(event);
    }
    events
}

struct TimeRange(NaiveDateTime, NaiveDateTime);

impl Iterator for TimeRange {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            let next = self.0 + Duration::minutes(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

fn get_guard_sleep_times(events: &Vec<Event>) -> HashMap<u32, Vec<NaiveDateTime>> {
    let mut sleep_times = HashMap::new();
    for window in events.windows(2) {
        let event1: &Event = &window[0];
        let event2: &Event = &window[1];
        let guard = event1.guard;
        match event1.action {
            Action::FallAsleep => {
                let minutes = sleep_times.entry(guard).or_insert(Vec::new());
                minutes.extend(TimeRange(event1.datetime, event2.datetime));
            }
            _ => (),
        }
    }
    sleep_times
}

fn get_guard_with_most_sleep(sleep_times: &HashMap<u32, Vec<NaiveDateTime>>) -> u32 {
    let guard = sleep_times
        .keys()
        .max_by_key(|g| sleep_times[g].len())
        .unwrap();
    *guard
}

fn get_minute_counter(sleep_times: &Vec<NaiveDateTime>) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();
    for &time in sleep_times {
        let count = counter.entry(time.minute()).or_insert(0);
        *count += 1;
    }
    counter
}

pub fn day4a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let records = get_records(&mut reader);
    let events = get_events(&records);
    let sleep_times = get_guard_sleep_times(&events);
    let guard_with_most_sleep = get_guard_with_most_sleep(&sleep_times);
    let minute_counter = get_minute_counter(&sleep_times[&guard_with_most_sleep]);
    let most_frequent_minute = minute_counter
        .keys()
        .max_by_key(|m| minute_counter[m])
        .unwrap();
    Ok(Answer::U(guard_with_most_sleep * most_frequent_minute))
}

fn get_sleep_frequencies(
    sleep_times: &HashMap<u32, Vec<NaiveDateTime>>,
) -> HashMap<u32, HashMap<u32, u32>> {
    let mut sleep_frequencies = HashMap::new();
    for (&guard, time_asleep) in sleep_times {
        sleep_frequencies.insert(guard, get_minute_counter(time_asleep));
    }
    sleep_frequencies
}

pub fn day4b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let records = get_records(&mut reader);
    let events = get_events(&records);
    let sleep_times = get_guard_sleep_times(&events);
    let sleep_frequencies = get_sleep_frequencies(&sleep_times);
    let guard = sleep_frequencies
        .keys()
        .max_by_key(|g| sleep_frequencies[g].values().max())
        .unwrap();
    let minute = sleep_frequencies[guard]
        .keys()
        .max_by_key(|m| sleep_frequencies[guard][m])
        .unwrap();
    Ok(Answer::U(guard * minute))
}
