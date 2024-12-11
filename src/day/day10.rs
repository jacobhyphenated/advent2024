use crate::util::vec2d::{Directions, Vec2d};

use super::Day;
use std::{collections::HashSet, fs};

/// Day 10: Hoof It
/// We need to reconstruct possible trails from a topographic map. The map (puzzle input)
/// contains heights represented by digits 0-9. A trail head starts at 0 and ends at 9,
/// and a good trail should increase by 1 for each step in the trail (no diagonals).
/// 
/// Part 1: Calculate the score of each trailhead, where the score is the number of 
/// destinations (9 tiles) accessible from that trailhead. Return the sum of all scores.
/// 
/// Part 2: The trails rating counts each possible path from the trailhead to a destination.
/// There may be multiple paths to the same destination. Return the sum of all trailhead ratings.
pub struct Day10;

impl Day<Vec2d<i32>> for Day10 {
    fn read_input() -> Vec2d<i32> {
        let input = fs::read_to_string("resources/day10.txt").expect("file day10.txt not found");
        parse_input(&input)
    }

    // Solved via breadth first search
    fn part1(input: &Vec2d<i32>) -> impl std::fmt::Display {
        let trail_starts = input.grid.iter()
            .enumerate()
            .filter(|(_, &digit)| digit == 0)
            .map(|(idx, _)| input.idx_to_point(idx))
            .collect::<Vec<_>>();
        let mut sum = 0;
        for start in trail_starts {
            let mut queue = Vec::new();
            queue.push(start);
            let mut end_points = HashSet::new();
            while let Some(current) = queue.pop() {
                if input[current] == 9 {
                    end_points.insert(current);
                    continue;
                }
                [Directions::Up, Directions::Down, Directions::Left, Directions::Right]
                    .into_iter()
                    .map(|direction| input.next_point(current, direction))
                    .flatten()
                    .filter(|&point| input[point] == input[current] + 1)
                    .for_each(|point| queue.push(point));
            }
            sum += end_points.len();
        }
        sum

    }

    // Very close to part 1, but greedily keep track of the number of trails while traversing them
    fn part2(input: &Vec2d<i32>) -> impl std::fmt::Display {
        let trail_starts = input.grid.iter()
            .enumerate()
            .filter(|(_, &digit)| digit == 0)
            .map(|(idx, _)| input.idx_to_point(idx))
            .collect::<Vec<_>>();
        let mut sum = 0;
        for start in trail_starts {
            let mut queue = Vec::new();
            queue.push(start);
            let mut num_trails = 1;
            while let Some(current) = queue.pop() {
                if input[current] == 9 {
                    continue;
                }
                let next_points = [Directions::Up, Directions::Down, Directions::Left, Directions::Right]
                    .into_iter()
                    .map(|direction| input.next_point(current, direction))
                    .flatten()
                    .filter(|&point| input[point] == input[current] + 1)
                    .collect::<Vec<_>>();
                
                // Count the number of times the trail branches into a new path
                // subtract if the branch hits a dead end
                if next_points.len() == 0 {
                    num_trails -= 1;
                } else if next_points.len() > 1 {
                    num_trails += next_points.len() - 1;
                }

                for p in next_points {
                    queue.push(p);
                }
            }
            sum += num_trails;
        }
        sum
    }
}

fn parse_input(input: &str) -> Vec2d<i32> {
    let grid = input.lines()
        .flat_map(|line| line.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .collect::<Vec<_>>()
        )
        .collect();
    let line_len = input.lines().next().unwrap().len();
    Vec2d {
        grid,
        line_len: line_len as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day10::part1(&input);
        assert_eq!("36", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day10::part2(&input);
        assert_eq!("81", result.to_string())
    }

}
