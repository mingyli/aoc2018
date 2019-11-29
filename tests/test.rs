extern crate aoc2018;
use aoc2018::{answer, config, run};

#[test]
fn test_day1a() {
    let config = config::Config {
        problem: "day1a".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(answer::Answer::I(470), run::run(&config).unwrap());
}

#[test]
fn test_day1b() {
    let config = config::Config {
        problem: "day1b".to_string(),
        filename: "./input/day1".to_string(),
    };
    assert_eq!(answer::Answer::I(790), run::run(&config).unwrap());
}

#[test]
fn test_day2a() {
    let config = config::Config {
        problem: "day2a".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!(answer::Answer::U(6696), run::run(&config).unwrap());
}

#[test]
fn test_day2b() {
    let config = config::Config {
        problem: "day2b".to_string(),
        filename: "./input/day2".to_string(),
    };
    assert_eq!(
        answer::Answer::S("bvnfawcnyoeyudzrpgslimtkj".to_string()),
        run::run(&config).unwrap()
    );
}

#[test]
fn test_day3a() {
    let config = config::Config {
        problem: "day3a".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!(answer::Answer::U(107820), run::run(&config).unwrap());
}

#[test]
fn test_day3b() {
    let config = config::Config {
        problem: "day3b".to_string(),
        filename: "./input/day3".to_string(),
    };
    assert_eq!(
        answer::Answer::S("661".to_string()),
        run::run(&config).unwrap()
    );
}
