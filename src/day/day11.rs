use super::Day;
use std::{collections::HashMap, fs};

/// Day 11: Plutonian Pebbles
/// 
/// A line of rocks changes every time you blink. It changes according to the rules
/// * If the rock is 0, it becomes 1
/// * If the rock has an even number of digits, it splits into two rocks.
///     ex: `22 -> 2 2` or 9908 -> 99 8
/// * Otherwise the rock becomes itself * 2024
/// 
/// Part 1: How many rocks exist if you blink 25 times?
/// 
/// Part 2: How many rocks exist if you blink 75 times?
pub struct Day11;

impl Day<Vec<i64>> for Day11 {
    fn read_input() -> Vec<i64> {
        let input = fs::read_to_string("resources/day11.txt").expect("file day11.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<i64>) -> impl std::fmt::Display {
        count_rocks(input, 25)
    }

    fn part2(input: &Vec<i64>) -> impl std::fmt::Display {
        count_rocks(input, 75)
    }
}

/// Because this is an exponential growth problem, maintaining a straight list of rocks doesn't work.
/// But rock numbers will repeat, and there will be multiples of the same rocks at a given time.
/// Instead of a list of all rocks, keep of count of the different rock values that exist
fn count_rocks(rocks: &[i64], blinks: i64) -> i64 {
    let mut memo: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut rock_counts = rocks.iter()
        .map(|&r| (r, 1)) // start with 1 of each rock
        .collect::<HashMap<_,_>>();

    for _ in 0 .. blinks {
        let mut updated_counts = HashMap::new();
        for rock in rock_counts.keys() {
            let current_count = rock_counts[rock];
            for &new_rock in blink_rock(*rock, &mut memo) {
                *updated_counts.entry(new_rock).or_insert(0) += current_count;
            }
        }
        rock_counts = updated_counts;
    }
    rock_counts.values().sum()
}

/// Calculate the next rock or rocks that exist after a blink from the passed in rock
/// 
/// The `memo` here is left over from a failed DFS implementation. It's probably not necessary
/// as all it does is remember the result of a single blick applied to a i32.
/// But it's staying because I made it work with lifetimes and it probably saves a few ms overall.
fn blink_rock<'a>(rock: i64, memo: &'a mut HashMap<i64, Vec<i64>>) -> &'a Vec<i64> {
    if !memo.contains_key(&rock) {
        let blinked = if rock == 0 {
            vec![rock + 1]
        } else if rock.to_string().len() % 2 == 0 {
            let rock_string = rock.to_string();
            vec![&rock_string[.. rock_string.len() / 2], &rock_string[rock_string.len() / 2 ..]].into_iter()
                .map(|s| s.parse().unwrap())
                .collect()
        } else {
            vec![rock * 2024]
        };
        memo.insert(rock, blinked);
    }
    return &memo[&rock];
} 

fn parse_input(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input("125 17");
        let result =  Day11::part1(&input);
        assert_eq!("55312", result.to_string())
    }

}
