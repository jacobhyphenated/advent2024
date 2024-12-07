use super::Day;
use std::fs;

type Calibration = (i64, Vec<i64>);

/// Day 7: Bridge Repair
/// 
/// The puzzle input is a list of incomplete equations.
/// The right side is the result, and the left is the numbers involved in the equation.
/// These numbers are evaluated on a left to right basis (no order of operations).
/// 
/// Part 1: Using either the `+` or `*`` operations between each number, is it possible
/// to make the left and right sides of the equation work. Return the sum of valid equations.
/// 
/// Part 2: There is an additional operator (`||`) called concat. This combines the left and right
/// numbers ex: `15 || 80 == 1580`. Sum the valid equations.
pub struct Day7;

impl Day<Vec<Calibration>> for Day7 {
    fn read_input() -> Vec<Calibration> {
        let input = fs::read_to_string("resources/day7.txt").expect("file day7.txt not found");
        parse_input(&input)
    }

    // Slightly smart brute force approach
    fn part1(input: &Vec<Calibration>) -> impl std::fmt::Display {
        input.iter()
            .filter(|(result, operations)| {
                try_operations(*result, operations[0], &operations[1..])
            })
            .map(|(r, _)| *r)
            .sum::<i64>()
    }

    fn part2(input: &Vec<Calibration>) -> impl std::fmt::Display {
        input.iter()
            .filter(|(result, operations)| {
                try_ops_concat(*result, operations[0], &operations[1..])
            })
            .map(|(r, _)| *r)
            .sum::<i64>()
    }
}

// Try all possible combinations of operators, but bail out / short circuit aggressively
fn try_operations(result: i64, current: i64, remaining: &[i64]) -> bool {
    if current > result {
        return false;
    }
    let next = remaining[0];
    if remaining.len() == 1 {
        if current + next == result || current * next == result {
            return true;
        } else {
            return false;
        }
    }
    let next_remaining = &remaining[1..];
    try_operations(result, next + current, next_remaining) 
        || try_operations(result, next * current, next_remaining)
}

// It's possible to combine parts 1 and 2 into one function, they are very similar,
// but I didn't bother for this problem.
fn try_ops_concat(result: i64, current: i64, remaining: &[i64]) -> bool {
    if current > result {
        return false;
    }
    let next = remaining[0];
    let concat = format!("{}{}", current.to_string(), next.to_string()).parse::<i64>().unwrap();
    if remaining.len() == 1 {
        if current + next == result || current * next == result || concat == result {
            return true;
        } else {
            return false;
        }
    }
    let next_remaining = &remaining[1..];
    try_ops_concat(result, next + current, next_remaining) 
        || try_ops_concat(result, next * current, next_remaining)
        || try_ops_concat(result, concat, next_remaining)
}

fn parse_input(input: &str) -> Vec<Calibration> {
    input.lines().map(|line|{
        let c = line.split(": ").collect::<Vec<_>>();
        let result = c[0].trim().parse().unwrap();
        let operations = c[1].split_whitespace()
            .map(|o| o.parse().unwrap())
            .collect();
        (result, operations)
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day7::part1(&input);
        assert_eq!("3749", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day7::part2(&input);
        assert_eq!("11387", result.to_string())
    }

}