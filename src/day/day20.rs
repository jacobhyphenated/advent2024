use super::Day;
use crate::util::grid::prelude::*;
use std::cmp::Ordering;
use std::fs;
use std::collections::BinaryHeap;

/// Day 20: Race Condition
/// 
/// You are trying to find the fastest way through a 2d maze.
/// However, you are allowed to cheat once. There is only one main path through the maze,
/// but when cheating, many new paths open up.
/// 
/// Part 1: You can, one time only, pass through a wall (becoming incorporeal for 2 moves)
/// Count how many possible solutions to the maze exist where cheating will allow you to finish
/// at least 100 moves faster than the solution without cheating.
/// 
/// Part 2: Now when you cheat, you become incorporeal for at most 20 spaces. You do not need
/// to use all 20 moves, but you can still only cheat once. The spot where you re-materialize
/// counts as one possible path (if you take multiple 20 step paths to the same destination,
/// it still only counts once). Now how many solutions finish the maze at least 100 moves faster?
pub struct Day20;

const DIRECTIONS: [Directions; 4] = [Directions::Down, Directions::Up, Directions::Left, Directions::Right];

impl Day<Vec2d<char>> for Day20 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day20.txt").expect("file day20.txt not found");
        parse_input(&input)
    }

    // Solved using lots and lots of dijkstra. But it's pretty speedy.
    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start = input.find(&'S').unwrap();
        let end = input.find(&'E').unwrap();
         // Full dijkstra distance map from END to all points.
        let dijkstra_map = dijstra_map(end, input);
        let max_time = dijkstra_map[input.point_to_idx(start)] - 100;

        // Now we'll traverse the maze using dijstra staring at the start point
        let mut distances = vec![i32::MAX; input.grid.len()];
        distances[input.point_to_idx(start)] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Node { cost: distances[input.point_to_idx(start)], position: start });

        let mut total_solutions = 0;
        while let Some(current) = queue.pop() {

            // Short circuit stop once we've exceeded our max time
            if current.cost > max_time {
                continue;
            }
            if current.cost > distances[input.point_to_idx(current.position)] {
                continue;
            }

            for direction in DIRECTIONS {
                let Some(next_pos) = input.next_point(current.position, direction) else {
                    continue;
                };
                if input[next_pos] == '#' {
                    // For walls, attempt to cheat. If cheating is possible,
                    // look up the path cost from the new post-cheat position
                    let Some(cheat_pos) = input.next_point(next_pos, direction) else {
                        continue;
                    };
                    let cheat_idx= input.point_to_idx(cheat_pos);
                    if input[cheat_pos] != '#' && distances[cheat_idx] > current.cost + 2 {
                        let cheat_cost = current.cost + 2 + dijkstra_map[cheat_idx];
                        if cheat_cost <= max_time {
                            // If cheating gets us to the finish in under the upper time limit, count it
                            total_solutions += 1;
                        }
                    }
                } else {
                    // For open spaces, use the standard dijkstra algorithm
                    let next_idx = input.point_to_idx(next_pos);
                    if current.cost + 1 < distances[next_idx] {
                        let next = Node { cost: current.cost + 1, position: next_pos };
                        distances[input.point_to_idx(next_pos)] = next.cost;
                        queue.push(next);
                    }
                }
            }
        }
        total_solutions
    }

    // Solved the same way as part 1, except we cheat in a different way
    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start = input.find(&'S').unwrap();
        let end = input.find(&'E').unwrap();

        // Full dijkstra distance map from END to all points.
        let dijkstra_map = dijstra_map(end, input);
        let max_time = dijkstra_map[input.point_to_idx(start)] - 100;

        let mut distances = vec![i32::MAX; input.grid.len()];
        distances[input.point_to_idx(start)] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Node { position: start, cost: 0 });

        let mut total_solutions = 0;
        while let Some(current) = queue.pop() {
            if current.cost > max_time {
                continue;
            }
            if current.cost > distances[input.point_to_idx(current.position)] {
                continue;
            }

            for direction in DIRECTIONS {
                let Some(next_pos) = input.next_point(current.position, direction) else {
                    continue;
                };
                if input[next_pos] != '#' {
                    // For open spaces, use the standard dijkstra algorithm
                    let next_idx = input.point_to_idx(next_pos);
                    if current.cost + 1 < distances[next_idx] {
                        let next = Node { cost: current.cost + 1, position: next_pos };
                        distances[input.point_to_idx(next_pos)] = next.cost;
                        queue.push(next);
                    }
                }
            }
            // Always try to cheat from any point we traverse using our dijstra pathfinding algorithm
            // First, examine all points that are within a manhattan distance of 20
            for x in current.position.x - 20 ..= current.position.x + 20 {
                let y_range = 20 - i32::abs(current.position.x - x);
                for y in current.position.y - y_range ..= current.position.y + y_range {
                    let cheat_point = Point::new(x, y);
                    let manhattan = cheat_point.manhattan_distance(&current.position);
                    // our position after cheating should be in bounds and not a wall
                    if cheat_point != current.position && input.in_bounds(cheat_point) && input[cheat_point] != '#' && manhattan <= 20 {
                        // constant time lookup for how far away the end is from our cheat position
                        let cheat_solve = current.cost + manhattan + dijkstra_map[input.point_to_idx(cheat_point)];
                        if cheat_solve <= max_time {
                            total_solutions += 1;
                        }
                    }
                }
            }
        }
        total_solutions
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    cost: i32,
    position: Point,
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

/// This function returns a dijkstra map of distances from the end point to all other maze points.
/// This is a useful way to memoize the distances from any point in the maze to the end
fn dijstra_map(end: Point, grid: &Vec2d<char>) -> Vec<i32> {
    let mut distances = vec![i32::MAX; grid.grid.len()];
    distances[grid.point_to_idx(end)] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Node { position: end, cost: 0 });

    while let Some(current) = queue.pop() {
        if current.cost > distances[grid.point_to_idx(current.position)] {
            continue;
        }
        [Directions::Up, Directions::Down, Directions::Left, Directions::Right].into_iter()
            .filter_map(|direction| grid.next_point(current.position, direction))
            .filter(|&next_pos| grid[next_pos] != '#')
            .for_each(|next_pos| {
                let next_idx = grid.point_to_idx(next_pos);
                if current.cost + 1 < distances[next_idx] {
                    let next = Node { cost: current.cost + 1, position: next_pos };
                    distances[grid.point_to_idx(next_pos)] = next.cost;
                    queue.push(next);
                }
            });
    }
    distances
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
