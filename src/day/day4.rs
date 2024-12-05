use super::Day;
use std::fs;
use crate::util::vec2d::{Vec2d, Directions, Point};

/// Day 4: Ceres Searc
/// 
/// The puzzle input is a 2d word search with the letters X, M, A, and S
/// 
/// Part 1: Find all occurances of XMAS. XMAS can be in all directions including diagonas
/// as well as spelled forward of backward and can overlap other XMAS words.
/// 
/// Part 2: Serach for a Diagonal MAS in an X shape such that
/// two MAS or backwards SAM intersect on the A character. example:
/// ```
/// M . S
/// . A .
/// M . S
/// ```
pub struct Day4;

impl Day<Vec2d<char>> for Day4 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day4.txt").expect("file day4.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        input.grid.iter().enumerate()
            .filter(|(_, c)| **c == 'X')
            .map(|(x_index, _)| {
                let x_point = input.idx_to_point(x_index);
                four_letter_list(x_point, input).into_iter()
                    .filter(|word| word == "XMAS")
                    .count()
            })
            .sum::<usize>()
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        input.grid.iter().enumerate()
            .filter(|(_, c)| **c == 'A')
            .filter(|(a_index, _)| is_diagonal(*a_index, input))
            .count()

    }
}

fn four_letter_list(start: Point, grid: &Vec2d<char>) -> Vec<String> {
    const DIRECTIONS: [Directions; 8] = [Directions::Up, Directions::Down, Directions::Left, Directions::Right,
            Directions::DownLeft, Directions::DownRight, Directions::UpLeft, Directions::UpRight];
    DIRECTIONS.into_iter().map(|direction| {
        let mut current = Some(start);
        let mut word = vec![current];
        for _ in 0 .. 3 {
            if let Some(point) = current {
                let next = grid.next_point(point, direction);
                current = next;
                word.push(current);
            } else {
                break;
            }
        }
        word.into_iter()
            .flatten() // get rid of nulls
            .map(|w| grid[w])
            .collect::<String>()
    })
    .collect()
}

fn is_diagonal(start: usize, grid: &Vec2d<char>) -> bool {
    let a_point = grid.idx_to_point(start);
    let diagonals = [Directions::UpLeft, Directions::UpRight, Directions::DownLeft, Directions::DownRight].into_iter()
        .filter_map(|d| grid.next_point(a_point, d))
        .collect::<Vec<_>>();
    if diagonals.len() != 4 {
        return false;
    } 
    if let [up_left, up_right, down_left, down_right] = &diagonals[0..4] {
        let left = [up_left, &a_point, down_right].into_iter()
            .map(|p| grid[*p])
            .collect::<String>();
        let right = [up_right, &a_point, down_left].into_iter()
            .map(|p| grid[*p])
            .collect::<String>();
        (left == "MAS" || left == "SAM") && (right == "MAS" || right == "SAM")
    } else {
        // Annoying, but even though I checked the len of 4,
        // I still need to provide the else case here for the compiler to be happy
        false
    }
}

fn parse_input(input: &str) -> Vec2d<char>{
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

    const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day4::part1(&input);
        assert_eq!("18", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day4::part2(&input);
        assert_eq!("9", result.to_string())
    }

}