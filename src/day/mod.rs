mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;

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
    println!("Day {day}:");
    match day {
        1 => Day1::run(),
        2 => Day2::run(),
        3 => Day3::run(),
        4 => Day4::run(),
        5 => Day5::run(),
        6 => Day6::run(),
        7 => Day7::run(),
        8 => Day8::run(),
        9 => Day9::run(),
        10 => Day10::run(),
        11 => Day11::run(),
        12 => Day12::run(),
        13 => Day13::run(),
        14 => Day14::run(),
        15 => Day15::run(),
        16 => Day16::run(),
        17 => Day17::run(),
        18 => Day18::run(),
        19 => Day19::run(),
        20 => Day20::run(),
        21 => Day21::run(),
        22 => Day22::run(),
        _ => println!("Day {day} not implemented"),
    }
}

