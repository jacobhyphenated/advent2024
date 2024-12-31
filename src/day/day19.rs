use super::Day;
use std::{collections::HashMap, fs};

/// Day 19: Linen Layout
/// 
/// There is an infinite supply of towels that come in descrete preset patterns.
/// This is the first part of the puzzle input.
/// 
/// The second part of the puzzle input are desirable patterns that could be composed of
/// different combonations of towels. It's possible that some desirable patterns cannot be
/// made with the given supply of towels.
/// 
/// Part 1: How many patterns can be composed from the supply of towels?
/// 
/// Part 2: How many possible combonations of towels exist to make the patterns?
pub struct Day19;

pub type Towels = (Vec<String>, Vec<String>);

impl Day<Towels> for Day19 {
    fn read_input() -> Towels {
        let input = fs::read_to_string("resources/day19.txt").expect("file day19.txt not found");
        parse_input(&input)
    }

    // Solved in the same way as part 2.
    // This could be done A LOT faster, but I solved the hard part for part 2 first,
    // and it ended up being speedy enough that it wasn't worth doing short circuit implementation for part 1
    fn part1(input: &Towels) -> impl std::fmt::Display {
        let (supply, patterns) = input;
        let mut memo = HashMap::new();
        patterns.iter()
            .map(|pattern| count_patterns(supply, pattern, &mut memo))
            .filter(|&count| count > 0)
            .count()
    }

    fn part2(input: &Towels) -> impl std::fmt::Display {
        let (supply, patterns) = input;
        let mut memo = HashMap::new();
        patterns.iter()
            .map(|pattern| count_patterns(supply, pattern, &mut memo))
            .sum::<usize>()
    }
}

/// Sove via recursive depth first search with memoization.
/// The memoization is absolutely essential to eliminate expensive repeating recursive calls
fn count_patterns<'a>(supply: &Vec<String>, pattern: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    // we've reached the end of the pattern. That means we have a success
    if pattern.is_empty() {
        return 1;
    }
    if let Some(val) = memo.get(pattern) {
        return *val;
    }

    // loop through each towel type in the supply. If the pattern starts with this towel,
    // create a recursive branch to find all possible combos of that towel + the rest of the pattern.
    let mut count = 0;
    for towel in supply {
        if pattern.starts_with(towel) {
            let valid_count = count_patterns(supply, &pattern[towel.len()..], memo);
            memo.insert(&pattern[towel.len()..], valid_count);
            count += valid_count;
        }
    }
    count
}

fn parse_input(input: &str) -> Towels {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let supply = parts[0].split(", ").map(ToString::to_string).collect::<Vec<_>>();
    let patterns = parts[1].lines().map(ToString::to_string).collect::<Vec<_>>();
    (supply, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day19::part1(&input);
        assert_eq!("6", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result = Day19::part2(&input);
        assert_eq!("16", result.to_string())
    }

}
