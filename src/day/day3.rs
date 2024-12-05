use regex::Regex;

use super::Day;
use std::fs;

/// Day 3: Mull It Over
/// 
/// The puzzle input is a broken program with corrupted characters
/// 
/// Part 1: A valid instruction is mul(2,4) which multiplies 2 by 4.
/// the numbers being multiplied are between 1 and 3 digits long.
/// instructions with invalide characters do nothing.
/// Add up the result of all valid mul instructions.
/// 
/// Part2: Additional instructions are `do()` and `don't()`
/// `do()` enables further instructions. `don't()` disables further instructions.
/// Assume the program start enabled. Sum the result of the enabled `mul()` operations.
pub struct Day3;

impl Day<String> for Day3 {
    fn read_input() -> String {
        fs::read_to_string("resources/day3.txt").expect("file day3.txt not found")
    }

    fn part1(input: &String) -> impl std::fmt::Display {
        let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
        re.captures_iter(input)
            .map(|capture| {
                let (_, [lhs, rhs]) = capture.extract();
                lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap()
            })
            .sum::<i32>()
    }

    fn part2(input: &String) -> impl std::fmt::Display {
        let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)|don\'t\(\)|do\(\)").unwrap();
        let mut on = true;
        let mut sum = 0;
        for capture in re.captures_iter(input) {
            // `capture.extract();` panics because of differing capture arguments for matchs
            let full_string = capture.get(0).unwrap().as_str();
            if full_string == "don't()" {
                on = false;
            } else if full_string == "do()" {
                on = true;
            } else if on {
                let lhs = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let rhs = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                sum += lhs * rhs;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result =  Day3::part1(&input);
        assert_eq!("161", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let result =  Day3::part2(&input);
        assert_eq!("48", result.to_string())
    }

}
