use crate::util::vec2d::{Directions, Vec2d};
use std::{collections::HashSet, fs};

use super::Day;

/// Day 6: Guard Gallivant
/// 
/// A guard is patrolling an area with open space `.` and obstacles '#'.
/// When guard travels in a straight line until it reaches an obstacle, then turns 90 degrees right.
/// 
/// Part 1: Predict the Guard's path until the guard leaves the area
/// How many spaces will the guard pass through?
/// 
/// Part 2: It is possible to add a new obstacle in such a way that the guard gets stuck in a loop.
/// How many locations on the map will cause this loop if an obstacle is added to just one space?
pub struct Day6;

impl Day<Vec2d<char>> for Day6 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day6.txt").expect("file day6.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start_pos = input.grid.iter().enumerate()
            .find(|(_, &c)| c == '^' )
            .map(|(idx, _)| idx)
            .unwrap();
        let mut guard_location = input.idx_to_point(start_pos);
        let mut direction = Directions::Up;
        let mut traversed = HashSet::new();
        traversed.insert(guard_location);
        loop {
            let Some(next) = input.next_point(guard_location, direction) else {
                break;
            };
            if input[next] == '#' {
                direction = rotate_right(direction);
            } else {
                guard_location = next;
                traversed.insert(next);
            }
        }
        traversed.len()
    }

    // So there should be a better way to do this.
    // Brute force checking each possible obstacle location is slow.
    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        input.grid.iter().enumerate()
            .filter(|(_, &c)| c == '.')
            .filter(|(idx, _)| {
                let mut test_obstruction = input.clone();
                test_obstruction.grid[*idx] = '#';
                is_guard_loop(test_obstruction)
            })
            .count()

    }
}

fn is_guard_loop(map: Vec2d<char>) -> bool {
    let start_pos = map.grid.iter().enumerate()
        .find(|(_, &c)| c == '^' )
        .map(|(idx, _)| idx)
        .unwrap();
    let mut guard_location = map.idx_to_point(start_pos);
    let mut direction = Directions::Up;
    let mut traversed = HashSet::new();
    traversed.insert((guard_location, direction));
    loop {
        let Some(next) = map.next_point(guard_location, direction) else {
            return false; // exited the map
        };
        if map[next] == '#' {
            direction = rotate_right(direction);
        } else {
            guard_location = next;
        }
        if !traversed.insert((guard_location, direction)) {
            // set already contained this value, we have a guard loop
            return true;
        }
    }
}

fn rotate_right(direction: Directions) -> Directions {
    match direction {
        Directions::Up => Directions::Right,
        Directions::Right => Directions::Down,
        Directions::Down => Directions::Left,
        Directions::Left => Directions::Up,
        _ => panic!("Direction {direction:?} not supported"),
    }
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

    const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day6::part1(&input);
        assert_eq!("41", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day6::part2(&input);
        assert_eq!("6", result.to_string())
    }

}
