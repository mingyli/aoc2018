use std::env;

mod answer;
mod config;
mod run;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args);

    let result = run::run(&config).unwrap();
    println!("result: {}", result);
}
