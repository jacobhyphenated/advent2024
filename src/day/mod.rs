mod day1;

use day1::Day1;
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
        println!("Part 1: {part1} {}ms", now.elapsed().as_nanos() as f64 / 1_000_000.0);
        let now = Instant::now();
        let part2 = Self::part2(&input);
        println!("Part 2: {part2} {}ms", now.elapsed().as_nanos() as f64 / 1_000_000.0);
    }
}

pub fn run(day: i32) {
    match day {
        1 => Day1::run(),
        _ => println!("Day {day} not implemented"),
    }
}

