use super::Day;
use crate::util::grid::prelude::*;
use std::cmp::Ordering;
use std::fs;
use std::collections::BinaryHeap;

/// Day 18: RAM Run
/// 
/// The puzzle input is a list of (x,y) coordinates. These are obstacles that are dropped sequentially
/// onto a 71 x 71 unit 2d grid (valid values are 0-70 inclusive).
/// 
/// You start in the top left (0,0) at attempt to get to the bottom right (70,70).
/// 
/// Part 1: Drop the first 1024 obstacles. How many moves (no diagonals) does it take to get to the end?
/// 
/// Part 2: Find the first point where there is no longer a valid path from start to end.
pub struct Day18;

impl Day<Vec<Point>> for Day18 {
    fn read_input() -> Vec<Point> {
        let input = fs::read_to_string("resources/day18.txt").expect("file day18.txt not found");
        input.lines()
            .map(|line| {
                let pts = line.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                Point::new(pts[0], pts[1])
            })
            .collect()
    }

    fn part1(input: &Vec<Point>) -> impl std::fmt::Display {
        let mut grid = Vec2d {
            grid: vec![true; 71 * 71],
            line_len: 71
        };
        for &point in input[..1024].iter() {
            grid[point] = false;
        }
        find_path(&grid).unwrap()
    }

    // Solve using a binary search. The binary search finishes at the first impassible grid
    fn part2(input: &Vec<Point>) -> impl std::fmt::Display {
        let mut valid_index = 1023;
        let mut invalid_index = input.len() - 1;
        while invalid_index - valid_index > 1 {
            let attempt_index = (valid_index + invalid_index) / 2;
            let mut grid = Vec2d {
                grid: vec![true; 71 * 71],
                line_len: 71
            };
            for &point in input[..=attempt_index].iter() {
                grid[point] = false;
            }
            let path = find_path(&grid);
            if path.is_some() {
                valid_index = attempt_index;
            } else {
                invalid_index = attempt_index;
            }
        }
        let first_bad_point = input[invalid_index];
        format!("{},{}", first_bad_point.x, first_bad_point.y)
    }
}

/// Use Dijkstra's algorithm to find the shortest path from start to end
fn find_path(grid: &Vec2d<bool>) -> Option<i32> {
    let start = Point::new(0, 0);
    let end = Point::new(70, 70);
    let mut distances = vec![i32::MAX; grid.grid.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Node { position: start, cost: 0 });
    distances[0] = 0;

    while let Some(current) = queue.pop() {
        if current.position == end {
            return Some(current.cost);
        }
        let current_idx = grid.point_to_idx(current.position);
        if current.cost > distances[current_idx] {
            continue;
        }
        [Directions::Up, Directions::Down, Directions::Left, Directions::Right].into_iter()
            .filter_map(|d| grid.next_point(current.position, d))
            .filter(|&point| grid[point])
            .for_each(|next_pos| {
                let next_idx = grid.point_to_idx(next_pos);
                let next_cost = current.cost + 1;
                if next_cost < distances[next_idx] {
                    queue.push(Node { position: next_pos, cost: next_cost });
                    distances[next_idx] = next_cost;
                }
            });
    }
    None
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    position: Point,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}