use crate::util::grid::prelude::*;

use super::Day;
use std::{collections::HashSet, fs};

/// Day 12: Garden Groups
/// 
/// The puzzle input is a grid of characters. These characters identidfy a plant in a garden
/// and can be grouped together by looking up, down, right, or left. There can be multiple
/// separate groupings of the same plant located in the input.
/// 
/// Part 1: For each grouping, find the area and the perimiter. Multiply together and sum.
/// 
/// Part 2: Instead of perimeter, use the number of sides in a the shape.
/// ```
/// .......
/// ..iii..  area = 9
/// ..iii..  perimeter = 12
/// ..iii..  sides = 4
/// .......
/// ```
pub struct Day12;

impl Day<Vec2d<char>> for Day12 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day12.txt").expect("file day12.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let regions = group_regions(input);
        regions.into_iter()
            .map(|region| region.len() * calc_perimeter(input, &region))
            .sum::<usize>()
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        let regions = group_regions(input);
        regions.into_iter()
            .map(|region| region.len() * calc_perimeter_sides(input, &region))
            .sum::<usize>()
    }
}

fn group_regions(input: &Vec2d<char>) -> Vec<HashSet<Point>> {
    let mut regions: Vec<HashSet<Point>> = Vec::new();
    for (idx, &c) in input.grid.iter().enumerate() {
        let point = input.idx_to_point(idx);
        if regions.iter().any(|region| region.contains(&point)) {
            continue;
        }
        let mut region = HashSet::new();
        let mut search = vec![point];
        while let Some(p) = search.pop() {
            region.insert(p);
            [Directions::Up, Directions::Down, Directions::Left, Directions::Right].into_iter()
                .filter_map(|direction| input.next_point(p, direction))
                .filter(|&neighbor| input[neighbor] == c && !region.contains(&neighbor))
                .for_each(|neighbor| search.push(neighbor));
        }
        regions.push(region);
    }
    regions
}

fn calc_perimeter(input: &Vec2d<char>, region: &HashSet<Point>) -> usize {
    let mut perimeter = 0;
    for &point in region {
        let c = input[point];
        // count all border spaces that are not the same character as the region
        perimeter += [Directions::Up, Directions::Down, Directions::Left, Directions::Right].into_iter()
            .map(|direction| input.next_point(point, direction))
            .filter(|border| border.map(|p| input[p]).unwrap_or('?') != c)
            .count();
    }
    perimeter
}

// determine the number of sides by calculating the changes in direction of the outside of the shape.
// do this by identifying corner spaces, and adding them up.
fn calc_perimeter_sides(input: &Vec2d<char>, region: &HashSet<Point>) -> usize {
    const CORNERS: [(Directions, Directions); 4] = [
        (Directions::Up, Directions::Left),
        (Directions::Up, Directions::Right),
        (Directions::Down, Directions::Left),
        (Directions::Down, Directions::Right),
    ];
    // an exterior corner is a point where the adjecent points in a corner direction are
    // outside of the region. A point may have multiple corners
    let exterior_corners = region.iter()
        .map(|&point| CORNERS.iter()
            .filter(|corner|{
                let c1 = input.next_unbounded(point, corner.0);
                let c2 = input.next_unbounded(point, corner.1);
                !region.contains(&c1) && !region.contains(&c2)
            })
            .count()
        )
        .sum::<usize>();

    // An interior corner is a point where the adjacent points in a corner direction are
    // inside the region, but the point diagonally in that direction is outside the region
    let interior_corners = region.iter()
        .map(|&point| CORNERS.iter()
            .filter(|corner|{
                let c1 = input.next_unbounded(point, corner.0);
                let c2 = input.next_unbounded(point, corner.1);
                let c_diag = input.next_unbounded(c1, corner.1);
                region.contains(&c1) && region.contains(&c2) && !region.contains(&c_diag)
            })
            .count()
        )
        .sum::<usize>();

    exterior_corners + interior_corners
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

    const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day12::part1(&input);
        assert_eq!("1930", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day12::part2(&input);
        assert_eq!("1206", result.to_string())
    }

}
