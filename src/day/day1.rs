use super::Day;
use std::fs;
pub struct Day1;

impl Day<(Vec<i32>, Vec<i32>)> for Day1 {
    fn read_input() -> (Vec<i32>, Vec<i32>) {
        let input = fs::read_to_string("resources/day1.txt").expect("file day1.txt not found");
        parse_input(&input)
    }

    fn part1(input: &(Vec<i32>, Vec<i32>)) -> impl std::fmt::Display {
        let (mut left, mut right) = input.clone();
        left.sort_unstable();
        right.sort_unstable();
        left.into_iter().zip(right)
            .map(|(a, b)| i32::max(a, b) - i32::min(a, b))
            .sum::<i32>()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn part2(input: &(Vec<i32>, Vec<i32>)) -> impl std::fmt::Display {
        let (left, right) = input;
        left.iter()
            .map(|lhs| {
                let count = right.iter()
                    .filter(|rhs| lhs == *rhs)
                    .count();
                lhs * count as i32 
            })
            .sum::<i32>()
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines()
        .map(|line| line
            .split_whitespace()
            .map(|item| item.parse().expect("Invalid Int"))
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<_>>();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        left.push(line[0]);
        right.push(line[1]);
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST_INPUT);
        let result =  Day1::part1(&input);
        assert_eq!("11", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST_INPUT);
        let result =  Day1::part2(&input);
        assert_eq!("31", result.to_string())
    }

}