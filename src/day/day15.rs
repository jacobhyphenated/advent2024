use crate::util::grid::prelude::*;

use super::Day;
use std::fs;

/// Day 15: Warehouse Woes
/// 
/// A robot is moving throughout the warehouse and moving boxes.
/// The puzzle input has two parts, the first describing the layout of the warehouse,
/// and the second as a list of movements the robot will attempt to take.
/// 
/// If the movement instruction moves the robot into a wall, nothing happens.
/// If the instruction moves the robot into a box, the robot will attempt to push the box and move.
/// If there are multiple boxes lined up, and an empty space beyond them, all those boxes will move.
/// 
/// Part 1: Run through the instructions. Find the location of all the boxes then
/// return the score which is 100 * y position + x position.
/// 
/// Part 2: The warehouse is actually twice as wide, and boxes take up two spaces horizontally.
/// The robot still takes up one space, but may push multiple boxes like so:
/// ```
/// ##############
/// ##......##..##
/// ##..........##
/// ##...[][]...##
/// ##....[]....##
/// ##.....@....##
/// ##############
/// ```
/// `^`
/// ```
/// ##############
/// ##......##..##
/// ##...[][]...##
/// ##....[]....##
/// ##.....@....##
/// ##..........##
/// ##############
/// ```
/// Return the score based on the first part of the box (`[`)
pub struct Day15;

pub type Warehouse = (Vec2d<char>, Vec<Directions>);

impl Day<Warehouse> for Day15 {
    fn read_input() -> Warehouse {
        let input = fs::read_to_string("resources/day15.txt").expect("file day15.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Warehouse) -> impl std::fmt::Display {
        let mut grid = input.0.to_owned();
        let mut robot = grid.grid.iter().enumerate()
            .filter(|(_, c)| **c == '@')
            .map(|(idx, _)| grid.idx_to_point(idx))
            .next().unwrap();
        for &movement in input.1.iter() {
            let Some(next) = grid.next_point(robot, movement) else {
                continue;
            };
            if grid[next] == '#' {
                continue;
            }
            if grid[next] == 'O' {
                move_box(next, &mut grid, movement);
            }
            if grid[next] == '.' {
                grid[next] = '@';
                grid[robot] = '.';
                robot = next;
            }
        }
        grid.grid.iter().enumerate()
            .filter(|&(_, c)| *c == 'O')
            .map(|(idx, _)| grid.idx_to_point(idx))
            .map(|point| point.y * 100 + point.x)
            .sum::<i32>()

    }

    fn part2(input: &Warehouse) -> impl std::fmt::Display {
        let (input_grid, instructions) = input;
        let updated_grid = input_grid.grid.iter()
            .flat_map(|&c| match c {
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                _ => panic!("Invalid grid character"),
            })
            .collect::<Vec<_>>();
        let mut grid = Vec2d {
            grid: updated_grid,
            line_len: input_grid.line_len * 2,
        };
        let mut robot = grid.grid.iter().enumerate()
            .filter(|(_, c)| **c == '@')
            .map(|(idx, _)| grid.idx_to_point(idx))
            .next().unwrap();

        for &movement in instructions {
            let Some(next) = grid.next_point(robot, movement) else {
                continue;
            };
            if grid[next] == '#' {
                continue;
            }
            if grid[next] == '[' || grid[next] == ']' {
                move_large_box(next, &mut grid, movement);
            }
            if grid[next] == '.' {
                grid[next] = '@';
                grid[robot] = '.';
                robot = next;
            }
        }
        grid.grid.iter().enumerate()
            .filter(|&(_, c)| *c == '[')
            .map(|(idx, _)| grid.idx_to_point(idx))
            .map(|point| point.y * 100 + point.x)
            .sum::<i32>()
    }
}

// This can be done recursively by greedily moving boxes that can be moved in the path
fn move_box(from: Point, grid: &mut Vec2d<char>, direction: Directions) -> bool {
    let Some(next) = grid.next_point(from, direction) else {
        return false;
    };
    if grid[next] == '.' {
        grid[next] = grid[from];
        grid[from] = '.';
        return true;
    } else if grid[next] == '#' {
        return false;
    } else if move_box(next, grid, direction) { // 'O'
        grid[next] = grid[from];
        grid[from] = '.';
        return true;
    }
    return false;
}

// We cannot greedily move the large box because there might be 2 independent
// upstream boxes that can or cannot be pushed individually, and we will
// only move this box if both upstream boxes can be pushed.
fn can_move_large_box(from: Point, grid: &Vec2d<char>, direction: Directions) -> bool {
    let other_from = match grid[from] {
        '[' => grid.next_unbounded(from, Directions::Right),
        ']' => grid.next_unbounded(from, Directions::Left),
        _ => return true,
    };
    let Some(next) = grid.next_point(from, direction) else {
        return false;
    };
    let Some(other_next) = grid.next_point(other_from, direction) else {
        return false;
    };
    if grid[next] == '#' || grid[other_next] == '#' {
        return false;
    }
    if grid[next] == '.' && grid[other_next] == '.' {
        return true;
    }
    can_move_large_box(next, grid, direction) && can_move_large_box(other_next, grid, direction)
}

// Left and right will work the same as before
// but we need additional checks for up and down pushing due to the box size
fn move_large_box(from: Point, grid: &mut Vec2d<char>, direction: Directions) -> bool {
    if direction == Directions::Left || direction == Directions::Right {
        return move_box(from, grid, direction);
    }
    let other_from = match grid[from] {
        '[' => grid.next_unbounded(from, Directions::Right),
        ']' => grid.next_unbounded(from, Directions::Left),
        _ => panic!("Trying to move something that is not a box"),
    };
    if can_move_large_box(from, grid, direction) {
        let next = grid.next_point(from, direction).unwrap();
        let other_next = grid.next_point(other_from, direction).unwrap();
        if grid[next] != '.' {
            move_large_box(next, grid, direction);
        }
        if grid[other_next] != '.' {
            move_large_box(other_next, grid, direction);
        }
        grid[next] = grid[from];
        grid[other_next] = grid[other_from];
        grid[from] = '.';
        grid[other_from] = '.';
        true
    } else {
        false
    }
}

fn parse_input(str: &str) -> Warehouse {
    let parts = str.split("\n\n").collect::<Vec<_>>();
    
    let chars = parts[0].lines()
        .flat_map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();
    let line_len = parts[0].lines().next().unwrap().len();
    let grid = Vec2d {
        grid: chars,
        line_len: line_len as i32,
    };

    let moves = parts[1].lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .map(|c| match c {
            '^' => Directions::Up,
            'v' => Directions::Down,
            '>' => Directions::Right,
            '<' => Directions::Left,
            _ => panic!("invalid direction character {c}"),
        })
        .collect();
    (grid, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day15::part1(&input);
        assert_eq!("10092", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day15::part2(&input);
        assert_eq!("9021", result.to_string())
    }

}
