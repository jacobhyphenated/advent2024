use crate::util::point::Point;

use super::Day;
use std::{collections::HashSet, fs};

/// Day 14: Restroom Redoubt
/// 
/// The puzzle input describes a list of robots with a position and a velocity.
/// The grid the robots operate on has a length of 101 and height of 103.
/// The velocity is in units per second.
/// When a robot reaches an edge of the grid, they wrap around to the other side.
/// Robots can overlap each other with no penalty.
/// 
/// Part 1: Find the position of all robots after the first 100 seconds.
/// Split the grid into 4 quadrants (and ignore the robots in the exact middles)
/// Count the number of robots in each quadrant, then multiply them.
/// 
/// Part 2: The robots have a hidden easter egg where they form a christmas tree.
/// Find the fewest number of seconds until that christmas tree appears.
pub struct Day14;

#[derive(Debug, Clone)]
pub struct Robot {
    position: Point,
    velocity: Point,
}

impl Day<Vec<Robot>> for Day14 {
    fn read_input() -> Vec<Robot> {
        let input = fs::read_to_string("resources/day14.txt").expect("file day14.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<Robot>) -> impl std::fmt::Display {
        let mut final_positions = Vec::new();
        for robot in input {
            let total_velocity = robot.velocity * 100;
            let final_position = total_velocity + robot.position;
            final_positions.push(Point::new(
                // Note: make sure to do euclid modulo instead of the `%` remainder operator
                final_position.x.rem_euclid(101),
                final_position.y.rem_euclid(103),
            ));
        }
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        for point in final_positions {
            if point.x < 50 && point.y < 51 {
                q1 += 1;
            } else if point.x > 50 && point.y < 51 {
                q2 += 1;
            } else if point.x < 50 && point.y > 51 {
                q3 += 1;
            } else if point.x > 50 && point.y > 51 {
                q4 += 1;
            }
        }
        q1 * q2 * q3 * q4
    }

    // Tried a couple of different approaches. This one worked:
    // assume the easter egg occurs when each robot is in a unique position.
    fn part2(input: &Vec<Robot>) -> impl std::fmt::Display {
        let mut seconds = 0;
        let mut updated_robots = input.to_owned();
        loop {
            seconds += 1;
            updated_robots = updated_robots.into_iter().map(|robot| {
                let next_position = robot.position + robot.velocity;
                Robot {
                    velocity: robot.velocity,
                    position: Point::new(
                        next_position.x.rem_euclid(101),
                        next_position.y.rem_euclid(103),
                    )
                }
            })
            .collect();
            let positions = updated_robots.iter()
                .map(|r| r.position)
                .collect::<HashSet<_>>();
            if updated_robots.len() == positions.len() {
                // Assume that for the xmas tree picture, all robots will be used in a unique position
                print_robots(&positions);
                return seconds;
            }
        }
    }
}

fn print_robots(robots: &HashSet<Point>) {
    for y in 0..103 {
        for x in 0..101 {
            if robots.contains(&Point::new(x,y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().map(|line|{
        let parts = line.split_whitespace()
            .map(|part| part.split('=').last().unwrap())
            .flat_map(|coord| coord.split(',').map(|i| i.parse::<i32>().unwrap()))
            .collect::<Vec<_>>();
        Robot {
            position: Point::new(parts[0], parts[1]),
            velocity: Point::new(parts[2], parts[3])
        }
    })
    .collect()
}
