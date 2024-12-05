use super::Day;
use std::fs;

/// Day 2: Red-Nosed Reports
/// 
/// The Puzzle input is multiple lines of numbers. Each line is a report.
/// Each number in the report is a level, read from left to right.
/// The report is Safe if 
///  - The levels are either all increasing or all decreasing
///  - Any two adjacent levels differ by at least one and at most three
/// 
/// Part 1: How many reports in the list are safe?
/// 
/// Part 2: The problem dampener allows a single level to be removed from a report.
/// How many reports are safe if one number can be removed from the report?
pub struct Day2;

impl Day<Vec<Vec<i32>>> for Day2 {
    fn read_input() ->  Vec<Vec<i32>> {
        let input = fs::read_to_string("resources/day2.txt").expect("file day2.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<Vec<i32>>) -> impl std::fmt::Display {
        input.iter()
            .filter(|report| Self::is_safe(report))
            .count()
    }

    fn part2(input: &Vec<Vec<i32>>) -> impl std::fmt::Display {
        input.iter()
            .filter(|report| Self::problem_dampener(report))
            .count()
    }
}

impl Day2 {
    fn is_safe(report: &[i32]) -> bool {
        let increasing = report[0] < report[1];
        for i in 1 .. report.len(){
            if report[i - 1] < report[i] && !increasing {
                return false;
            } else if report[i - 1] > report[i] && increasing {
                return false;
            } else if report[i - 1] == report[i] {
                return false;
            }
            
            if i32::abs(report[i] - report[i-1]) > 3 {
                return false;
            }
        }
        true
    }

    fn problem_dampener(report: &[i32]) -> bool {
        if Self::is_safe(report) {
            return true;
        }
        for i in 0..report.len() {
            let mut r = report.to_owned();
            r.remove(i);
            if Self::is_safe(&r) {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| 
        line.split_whitespace()
            .map(|s| s.parse().expect("invalid int"))
            .collect()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST_INPUT);
        let result =  Day2::part1(&input);
        assert_eq!("2", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST_INPUT);
        let result =  Day2::part2(&input);
        assert_eq!("4", result.to_string())
    }

}