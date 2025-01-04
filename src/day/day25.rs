use super::Day;
use std::fs;

/// Day 25: Code Chronicle
/// 
/// The puzzle input is a list of keys and locks as shown by their tumblers.
/// If the top row is filled in, it's a lock, if the bottom row is filled, it's a key.
/// 
/// Here's an example lock:
/// ```
/// #####
/// ##.##
/// .#.##
/// ...##
/// ...#.
/// ...#.
/// .....
/// ```
/// This lock's tumblers can be described as `[1,2,0,5,3]`
/// 
/// A key fits the lock if the key groves do not overlap the lock tumblers.
/// (note: they don't have to exactly match, just have to not overlap)
/// 
/// Part 1: Try every key in every lock. How many fit together?
pub struct Day25;

impl Day<(Vec<Vec<i32>>, Vec<Vec<i32>>)> for Day25 {
    fn read_input() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let input = fs::read_to_string("resources/day25.txt").expect("file day25.txt not found");
        parse_input(&input)
    }

    fn part1(input: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> impl std::fmt::Display {
        let (locks, keys) = input;
        let mut matches = 0;
        for key in keys {
            for lock in locks {
                // compare the key and lock. If the two don't overlap (sum less than 6)
                // for each position on the lock, then they fit
                if key.iter().zip(lock).all(|(top, bottom)| top + bottom <= 5) {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn part2(_: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> impl std::fmt::Display {
        "AOC 2024"
    }
}

// This is mostly a string parsing problem. Convert the key and lock inputs
// into a Vec<i32> describing the tumblers/grooves.
fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let grids = input.split("\n\n").collect::<Vec<_>>();
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for grid in grids {
        let lines = grid.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut grooves = Vec::new();
        for idx in 0 .. 5 {
            let count = lines.iter()
                .map(|line| line[idx])
                .filter(|&c| c == '#')
                .count() - 1;
            grooves.push(count.try_into().unwrap());
        }
        if lines[0][0] == '#' {
            locks.push(grooves);
        } else {
            keys.push(grooves);
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result = Day25::part1(&input);
        assert_eq!("3", result.to_string())
    }
}
