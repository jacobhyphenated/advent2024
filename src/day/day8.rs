use std::{collections::{HashMap, HashSet}, fs};

use crate::util::vec2d::{Point, Vec2d};

use super::Day;

/// Day 8: Resonant Collinearity
/// 
/// There are multiple antennae tuned to different frequency described in
/// the puzzle input. Each type (denoted by a single character) forms
/// Antinodes with other antennae of the same type.
/// 
/// An antinode occurs at any point that is perfectly in line with 2 antennas
/// of the same frequency, but only when one antenna is twice as far away from the other.
/// 
/// Example, where `#` is an antinode:
/// ```
/// ..........
/// ...#......
/// ..........
/// ....a.....
/// ..........
/// .....a....
/// ..........
/// ......#...
/// ..........
/// ..........
/// ```
/// 
/// Part 1: How many antinodes are on the grid area defined by the puzzle input?
/// Note that an antinode can exist on top of a different antenna.
/// 
/// Part 2: An antinode occurs at any grid position exactly in line with the two antennas,
/// including the location of the two antennas. How many antinodes?
pub struct Day8;

impl Day<Vec2d<char>> for Day8 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day8.txt").expect("file day8.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let antennae = find_antennae(input);
        let mut antinodes = HashSet::new();
        // nodes are grouped by antenna frequency
        for nodes in antennae.values() {
            if nodes.len() <= 1 {
                continue;
            }
            // compare each antenna of the same frequency to all the others
            for i in 0 .. nodes.len() - 1 {
                for j in i + 1 .. nodes.len() {
                    let diff = nodes[i] - nodes[j];
                    antinodes.insert(nodes[i] + diff);
                    antinodes.insert(nodes[j] - diff);
                }
            }
        }
        antinodes.into_iter()
            .filter(|&antinode | input.in_bounds(antinode))
            .count()
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        let antennae = find_antennae(input);
        let mut antinodes = HashSet::new();
        for nodes in antennae.values() {
            if nodes.len() <= 1 {
                continue;
            }
            for i in 0 .. nodes.len() - 1 {
                for j in i + 1 .. nodes.len() {
                    // Same as part 1, but continue until we reach the bounds edge of our grid
                    // also add the antennas themselves
                    antinodes.insert(nodes[i]);
                    antinodes.insert(nodes[j]);
                    let diff = nodes[i] - nodes[j];
                    let mut line = nodes[i] + diff;
                    while input.in_bounds(line) {
                        antinodes.insert(line);
                        line = line + diff;
                    }
                    line = nodes[j] - diff;
                    while input.in_bounds(line) {
                        antinodes.insert(line);
                        line = line - diff;
                    }
                }
            }
        }
        antinodes.len()
    }
}

fn find_antennae(input: &Vec2d<char>) -> HashMap<char, Vec<Point>> {
    let mut antennae = HashMap::new();
    for (idx, c) in input.grid.iter().enumerate() {
        if *c == '.' {
            continue;
        }
        if !antennae.contains_key(c) {
            antennae.insert(*c, Vec::new());
        }
        antennae.get_mut(c).unwrap().push(input.idx_to_point(idx));
    }
    antennae
}

fn parse_input(input: &str) -> Vec2d<char> {
    let chars = input.lines()
        .flat_map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();
    let line_len = input.lines().next().unwrap().len();
    Vec2d {
        grid: chars,
        line_len: line_len as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day8::part1(&input);
        assert_eq!("14", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day8::part2(&input);
        assert_eq!("34", result.to_string())
    }

}
