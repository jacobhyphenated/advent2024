mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;

use std::fmt::Display;
use std::time::Instant;

trait Day<T> {
    fn read_input() -> T;
    fn part1(input: &T) -> impl Display;
    fn part2(input: &T) -> impl Display;

    fn run() {
        let input = Self::read_input();
        let now = Instant::now();
        let part1 = Self::part1(&input);
        println!("Part 1: {part1} ({}ms)", now.elapsed().as_nanos() as f64 / 1_000_000.0);
        let now = Instant::now();
        let part2 = Self::part2(&input);
        println!("Part 2: {part2} ({}ms)", now.elapsed().as_nanos() as f64 / 1_000_000.0);
    }
}

pub fn run(day: i32) {
    match day {
        1 => Day1::run(),
        2 => Day2::run(),
        3 => Day3::run(),
        4 => Day4::run(),
        5 => Day5::run(),
        6 => Day6::run(),
        7 => Day7::run(),
        8 => Day8::run(),
        _ => println!("Day {day} not implemented"),
    }
}

