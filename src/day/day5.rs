use std::collections::{HashMap, HashSet};
use std::fs;
use super::Day;

type PrintEdits = (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>);

/// Day 5: Print Queue
/// 
/// The puzzle input is two parts.
/// 
/// The first part contains a list of number pairs `X|Y`. These rules
/// state that the edit for page number X must occur before page number Y.
/// 
/// The second part are multiple edit instructions, each containing a list of page numbers.
/// 
/// Part 1: Looking at each valid edit sequence determined by the edit rules,
/// sum the middle page numbers of each edit sequence.
/// 
/// Part 2: Fix the invalid edit sequences. Then sum the middle page numbers
/// of the corrected edit sequences (ignoring the originally valid edits)
pub struct Day5;

impl Day<PrintEdits> for Day5 {
    fn read_input() -> PrintEdits {
        let input = fs::read_to_string("resources/day5.txt").expect("file day5.txt not found");
        parse_input(&input)
    }

    fn part1(input: &PrintEdits) -> impl std::fmt::Display {
        let (rules, edits) = input;
        edits.iter()
            .filter(|edit| Self::is_valid_edit(edit, rules))
            .map(|edit| edit[edit.len() / 2])
            .sum::<i32>()
    }

    fn part2(input: &PrintEdits) -> impl std::fmt::Display {
        let (rules, edits) = input;
        edits.iter()
            .filter(|edit| !Self::is_valid_edit(edit, rules))
            .map(|edit| {
                let mut fixed = edit.clone();
                // Some of these will need multiple passes to fix
                while !Self::is_valid_edit(&fixed, rules) {
                    for i in 0 .. edit.len() - 1 {
                        // Look at the next two pages
                        // if they are being edited in the wrong order, swap them
                        let valid = rules.get(&fixed[i])
                            .map(|set| set.contains(&fixed[i+1]))
                            .unwrap_or(false);
                        if !valid {
                            let current = fixed[i];
                            fixed[i] = fixed[i+1];
                            fixed[i+1] = current;
                        }
                    }
                }
                fixed[fixed.len() / 2]
            })
            .sum::<i32>()
    }
}

impl Day5 {

    /// Look at every pair of two letters (using `windows(2)`)
    /// The left number should always have a rule entry requiring it
    /// to come before the right number.
    fn is_valid_edit(edit: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
        edit.windows(2).all(|slice| {
            rules.get(&slice[0])
                .unwrap_or(&HashSet::new())
                .contains(&slice[1])
        })
    }
}

fn parse_input(input: &str) -> PrintEdits {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let edits = split[1].lines()
        .map(|line| { 
            line.trim()
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect()
        })
        .collect();

    let mut rules = HashMap::new();
    for rule in split[0].lines() {
        let [lhs, rhs]: [i32; 2] = rule.split("|")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap(); // Force the vec into an array of size two for destructuring
        if !rules.contains_key(&lhs) {
            rules.insert(lhs, HashSet::new());
        }
        rules.get_mut(&lhs).unwrap().insert(rhs);
    }

    (rules, edits)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day5::part1(&input);
        assert_eq!("143", result.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day5::part2(&input);
        assert_eq!("123", result.to_string());
    }

}
