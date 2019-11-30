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

#[test]
fn test_day4a() {
    let config = config::Config {
        problem: "day4a".to_string(),
        filename: "./input/day4".to_string(),
    };
    assert_eq!(answer::Answer::U(94542), run::run(&config).unwrap());
}

#[test]
fn test_day4b() {
    let config = config::Config {
        problem: "day4b".to_string(),
        filename: "./input/day4".to_string(),
    };
    assert_eq!(answer::Answer::U(50966), run::run(&config).unwrap());
}

#[test]
fn test_day5a() {
    let config = config::Config {
        problem: "day5a".to_string(),
        filename: "./input/day5".to_string(),
    };
    assert_eq!(answer::Answer::US(11152), run::run(&config).unwrap());
}

#[test]
fn test_day5b() {
    let config = config::Config {
        problem: "day5b".to_string(),
        filename: "./input/day5".to_string(),
    };
    assert_eq!(answer::Answer::US(6136), run::run(&config).unwrap());
}

#[test]
fn test_day6a() {
    let config = config::Config {
        problem: "day6a".to_string(),
        filename: "./input/day6".to_string(),
    };
    assert_eq!(answer::Answer::US(3840), run::run(&config).unwrap());
}

#[test]
fn test_day6b() {
    let config = config::Config {
        problem: "day6b".to_string(),
        filename: "./input/day6".to_string(),
    };
    assert_eq!(answer::Answer::US(46542), run::run(&config).unwrap());
}
