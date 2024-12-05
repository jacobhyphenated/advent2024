#![warn(clippy::all, clippy::pedantic)]
mod day;
pub mod util;

use std::env;
use std::process;
use day::run;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage - list each day you want to run");
        println!("    to run days 1 and 15:");
        println!("    cargo run 1 15");
        process::exit(0);
    }
    let days = &args[1..];
    for day in days {
        if let Ok(day) = day.parse::<i32>() {
            run(day);
        } else {
            println!("Invalid argument: {day}");
        }
    }
}
