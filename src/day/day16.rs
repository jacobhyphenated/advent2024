use super::Day;
use crate::util::grid::prelude::*;
use std::fs;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Day 16: Reindeer Maze
/// 
/// The puzzle input represents a 2D maze where S is the starting position, and E is the end position.
/// Start facing in the right direction. Each step forward costs 1, and each 90 degree turn costs 1000.
/// 
/// Part 1: What is the lowest cost path to get to the end of the maze?
/// 
/// Part 2: There are multiple lowest cost solutions. How many total points on the maze are
/// traversed by all the possible lowest cost path solutions?
pub struct Day16;

impl Day<Vec2d<char>> for Day16 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day16.txt").expect("file day16.txt not found");
        parse_input(&input)
    }

    // Simple implementation of Dijkstra's algorithm to quickly find the best path through the maze
    // Note that we must track both position and direction as the same position might be crossed
    // from a separate direction with a very different cost score.
    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start = input.find(&'S').unwrap();
        let start_direction = Directions::Right;

        let mut distances:HashMap<(Point, Directions), i32> = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(Node { cost: 0, position: start, direction: start_direction, parent: None });
        distances.insert((start, start_direction), 0);

        while let Some(current) = queue.pop() {
            if input[current.position] == 'E' {
                return current.cost;
            }

            let current_cost = *distances.get(&(current.position, current.direction)).unwrap_or(&i32::MAX);
            if current.cost > current_cost {
                continue;
            }

            for next_direction in possible_directions(current.direction) {
                let Some(next_point) = input.next_point(current.position, next_direction) else {
                    continue;
                };
                if input[next_point] == '#' {
                    continue;
                }
                let next_cost = current.cost + 1 + if next_direction == current.direction { 0 } else { 1000 };
                if next_cost < *distances.get(&(next_point, next_direction)).unwrap_or(&i32::MAX) {
                    distances.insert((next_point, next_direction), next_cost);
                    queue.push(Node { cost: next_cost, position: next_point, direction: next_direction, parent: None });
                }
            }
        }
        0 // Did not find a path
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        let paths = best_paths(input);
        paths.into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    cost: i32,
    position: Point,
    direction: Directions,
    parent: Option<Box<Node>>, // used for part 2
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

// Modify the Dijkstra's algorithm from part 1. Now it does not exit when reaching the end point.
// Instead in continues to create map out paths and costs, but it does not allow a new path
// to exceed the least cost path (which we find first because dijkstra's algorithm is greedy).
// 
// Also changes our nodes to keep track of their parent so we can re-build the exact path taken.
fn best_paths(input: &Vec2d<char>) -> Vec<Vec<Point>> {
    let start = input.find(&'S').unwrap();
    let start_direction = Directions::Right;

    let mut distances:HashMap<(Point, Directions), i32> = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut best_paths = Vec::new();
    let mut best_cost: i32 = i32::MAX;
    queue.push(Node { cost: 0, position: start, direction: start_direction, parent: None });
    distances.insert((start, start_direction), 0);

    while let Some(current) = queue.pop() {
        if current.cost > best_cost {
            continue;
        }

        if input[current.position] == 'E' {
            best_cost = current.cost;
            let path = determine_path(current);
            best_paths.push(path);
            continue;
        }

        let current_cost = *distances.get(&(current.position, current.direction)).unwrap_or(&i32::MAX);
        // There is a better path, so this node cannot be on the best path
        if current.cost > current_cost {
            continue;
        }

        for next_direction in possible_directions(current.direction) {
            let Some(next_point) = input.next_point(current.position, next_direction) else {
                continue;
            };
            if input[next_point] == '#' {
                continue;
            }
            let next_cost = current.cost + 1 + if next_direction == current.direction { 0 } else { 1000 };
            if next_cost <= *distances.get(&(next_point, next_direction)).unwrap_or(&i32::MAX) {
                distances.insert((next_point, next_direction), next_cost);
                let next_node = Node { 
                    cost: next_cost, 
                    position: next_point, 
                    direction: next_direction,
                    parent: Some(Box::new(current.clone()))
                };
                queue.push(next_node.clone());
            }
        }
    }
    best_paths
}

fn possible_directions(direction: Directions) -> Vec<Directions> {
    match direction {
        Directions::Down => vec![Directions::Down, Directions::Left, Directions::Right],
        Directions::Left => vec![Directions::Left, Directions::Up, Directions::Down],
        Directions::Up => vec![Directions::Up, Directions::Left, Directions::Right],
        Directions::Right => vec![Directions::Right, Directions::Up, Directions::Down],
        _ => panic!("Unsupported direction: {direction:?}"),
    }
}

fn determine_path(end: Node) -> Vec<Point> {
    let mut path = vec![end.position];
    let mut current = end;
    while let Some(next) = current.parent {
        path.push(next.position);
        current = *next;
    }
    path.reverse();
    path
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

    const TEST: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day16::part1(&input);
        assert_eq!("7036", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day16::part2(&input);
        assert_eq!("45", result.to_string())
    }

}
