extern crate aoc2018;
use aoc2018::{config, run};

#[test]
fn test_day1a() {
    let config = config::Config {
        problem: "day1a".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(470, run::run(&config).unwrap());
}

#[test]
fn test_day1b() {
    let config = config::Config {
        problem: "day1b".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(790, run::run(&config).unwrap());
}

#[test]
fn test_day2a() {
    let config = config::Config {
        problem: "day2a".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!(6696, run::run(&config).unwrap());
}

#[test]
fn test_day2b() {
    let config = config::Config {
        problem: "day2b".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!("bvnfawcnyoeyudzrpgslimtkj", run::run(&config).unwrap());
}

#[test]
fn test_day3a() {
    let config = config::Config {
        problem: "day3a".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!(107820, run::run(&config).unwrap());
}

#[test]
fn test_day3b() {
    let config = config::Config {
        problem: "day3b".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!("661", run::run(&config).unwrap());
}

#[test]
fn test_day4a() {
    let config = config::Config {
        problem: "day4a".to_string(),
        filename: "./input/day4".to_string(),
    };
    assert_eq!(94542, run::run(&config).unwrap());
}

#[test]
fn test_day4b() {
    let config = config::Config {
        problem: "day4b".to_string(),
        filename: "./input/day4".to_string(),
    };
    assert_eq!(50966, run::run(&config).unwrap());
}

#[test]
fn test_day5a() {
    let config = config::Config {
        problem: "day5a".to_string(),
        filename: "./input/day5".to_string(),
    };
    assert_eq!(11152, run::run(&config).unwrap());
}

#[test]
fn test_day5b() {
    let config = config::Config {
        problem: "day5b".to_string(),
        filename: "./input/day5".to_string(),
    };
    assert_eq!(6136, run::run(&config).unwrap());
}

#[test]
fn test_day6a() {
    let config = config::Config {
        problem: "day6a".to_string(),
        filename: "./input/day6".to_string(),
    };
    assert_eq!(3840, run::run(&config).unwrap());
}

#[test]
fn test_day6b() {
    let config = config::Config {
        problem: "day6b".to_string(),
        filename: "./input/day6".to_string(),
    };
    assert_eq!(46542, run::run(&config).unwrap());
}

#[test]
fn test_day7a() {
    let config = config::Config {
        problem: "day7a".to_string(),
        filename: "./input/day7".to_string(),
    };
    assert_eq!("BFGKNRTWXIHPUMLQVZOYJACDSE", run::run(&config).unwrap());
}

#[test]
fn test_day7b() {
    let config = config::Config {
        problem: "day7b".to_string(),
        filename: "./input/day7".to_string(),
    };
    assert_eq!(1163, run::run(&config).unwrap());
}

#[test]
fn test_day8a() {
    let config = config::Config {
        problem: "day8a".to_string(),
        filename: "./input/day8".to_string(),
    };
    assert_eq!(42501, run::run(&config).unwrap());
}

#[test]
fn test_day8b() {
    let config = config::Config {
        problem: "day8b".to_string(),
        filename: "./input/day8".to_string(),
    };
    assert_eq!(30857, run::run(&config).unwrap());
}
