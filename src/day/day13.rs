use crate::util::grid::prelude::*;

use super::Day;
use std::fs;

/// Day 13: Claw Contraption
/// 
/// A claw machine has two buttons that move the claw a specific number of spaces along the x and y axis.
/// There is one prize in a defined location for each claw machine.
/// 
/// It costs 3 tokens to push the "A" button and 1 token to push the "B" button.
/// 
/// Part 1: For each claw machine where the prize can be reached, what is the minimum number of 
/// tokens needed to reach the prize? Sum this number for all claw machines.
/// 
/// Part 2: Actually, the prize is located an additiona `10,000,000,000,000` further in the x and y directions.
pub struct Day13;

#[derive(Debug)]
pub struct Claw {
    button_a: Point64,
    button_b: Point64,
    prize: Point64,
}

impl Day<Vec<Claw>> for Day13 {
    fn read_input() -> Vec<Claw> {
        let input = fs::read_to_string("resources/day13.txt").expect("file day13.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<Claw>) -> impl std::fmt::Display {
        input.iter()
            .filter_map(linear_algebra)
            .sum::<i64>()
    }

    fn part2(input: &Vec<Claw>) -> impl std::fmt::Display {
        let offset: i64 = 10_000_000_000_000;
        input.iter()
            .map(|claw| {
                Claw {
                    button_a: claw.button_a,
                    button_b: claw.button_b,
                    prize: claw.prize + offset
                }
            })
            .filter_map(|claw| linear_algebra(&claw))
            .sum::<i64>()
    }
}

/// This problem can be solved using linear algebra. Consider the following matrix:
/// ```
/// [ax bx | px]
/// [ay by | py]
/// ```
/// Button a = (ax, ay), button b = (bx, by) and the prize = (px, py).
/// 
/// If we reduce the matrix, we get:
/// ```
/// [1 0 | a_presses]
/// [0 1 | b_presses]
/// ```
/// where a_presses and b_presses are whole numbers in a solvable claw machine
fn linear_algebra(claw: &Claw) -> Option<i64> {
    let (mut ax, mut ay) = claw.button_a.to_f64();
    let (bx, mut by) = claw.button_b.to_f64();
    let (mut px, mut py) = claw.prize.to_f64();

    let ay_next = ay - ax * ay / ax;
    let by_next = by - bx * ay / ax;
    let py_next = py - px * ay / ax;
    ay = ay_next;
    by = by_next;
    py = py_next;

    let ax_next = ax - ay * bx / by;
    let px_next = px - py * bx / by;
    ax = ax_next;
    px = px_next;

    px /= ax;
    py /= by;

    // round will account for small floating point errors
    let a_presses = px.round() as i64;
    let b_presses = py.round() as i64;

    // Check if this has a working solution. A fractional number would fail after rounding
    if claw.button_a * a_presses + claw.button_b * b_presses == claw.prize {
        Some(3 * a_presses + b_presses)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<Claw> {
    input.split("\n\n")
        .map(|claw_string| {
            let lines = claw_string.lines().collect::<Vec<_>>();
            let parse_button = |line_str: &str| {
                let point = line_str.split(": ").last().unwrap()
                    .split(", ")
                    .map(|pt| pt.split('+').last().unwrap().parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                Point64::new(point[0], point[1])
            };
            let button_a = parse_button(lines[0]);
            let button_b = parse_button(lines[1]);
            let prize = lines[2].split(": ").last().unwrap()
                .split(", ")
                .map(|pt| pt.split('=').last().unwrap().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let prize = Point64::new(prize[0], prize[1]);
            Claw {
                button_a,
                button_b,
                prize
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day13::part1(&input);
        assert_eq!("480", result.to_string())
    }

}
