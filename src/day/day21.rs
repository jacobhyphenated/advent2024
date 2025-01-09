use std::collections::{BinaryHeap, HashMap};
use std::fs;
use super::Day;
use crate::util::grid::prelude::*;

/// Day 21: Keypad Conundrum
/// 
/// A keypad has 10 possible digits layed out as follows:
/// ```
/// 7 8 9
/// 4 5 6
/// 1 2 3
///   0 A
/// ```
/// A robot is necessary to press the buttons. This robot has a control
/// pad that moves its robotic arm. The control pad looks as follows:
/// ```
///   ^ A
/// < v >
/// ```
/// 
/// * Robots start with their arms pointed a the `A` or Activate key.
/// * The robotic arm can never traverse the empty space.
/// 
/// The puzzle input is a list of codes that must be typed on the numeric keypad
/// such as: `029A`. In this example, the robot would need to press the `<` key,
/// then the `A` key, to move the arm from the `A` to the `0`, then press `0`.
/// Then the next sequence of instructions to reach to remaining digits.
/// 
/// However, the keypad for the robot is also inaccessible, and another robot
/// is required to use its arm to manipulate the first robots keybad.
/// 
/// Part 1: In total, there is:
/// * one directional keypad operated by you.
/// * two directional keypads operated by robots.
/// * one numeric keypad operated by a robot
/// 
/// Find the minimum number of key presses you must make to type out the numeric code.
/// Multiply that number by the numeric part of the code (`029A` would be `29`).
/// Sum this number up for each code in the puzzle input.
/// 
/// Part 2: There are actually 25 robots operating directional keypads
/// (plus you and the numeric keypad robot). Using this chain of robots,
/// calculate the compexity score in the same way as part 1.
pub struct Day21;

impl Day<Vec<String>> for Day21 {
    fn read_input() -> Vec<String> {
        let input = fs::read_to_string("resources/day21.txt").expect("file day21.txt not found");
        parse_input(&input)
    }

    // We'll sovle part 1 and part 2 in the same general way.
    fn part1(input: &Vec<String>) -> impl std::fmt::Display {
        solve_for_robot_chain(2, input)
    }

    fn part2(input: &Vec<String>) -> impl std::fmt::Display {
        solve_for_robot_chain(25, input)
    }
}

// function to set up the robot chains and calculate the final result
fn solve_for_robot_chain(length: i32, input: &[String]) -> i64 {
    // There are actually only 2 "robot" objects that will be borrowed by all the robot chains
    // the keypad robot, and the directional robot, are built here
    let numeric_keypad = Vec2d {
        grid: vec!['7','8','9','4','5','6', '1', '2', '3', 'X', '0', 'A'],
        line_len: 3,
    };
    let direction_keypad = Vec2d {
        grid: vec!['X','^','A','<','v','>'],
        line_len: 3,
    };
    let mut key_robot = Robot::new(numeric_keypad);
    let mut direction_robot = Robot::new(direction_keypad);
    key_robot.load_all_keys();
    direction_robot.load_all_keys();

    // Build the robot chain including [`length`] nested directional robots
    let mut parent =  RobotState {
        robot: &direction_robot,
        current_pos: direction_robot.find_key_pos('A'),
        level: 0,
        parent: Box::new(None),
    };
    for level in 1 ..= length {
        parent = RobotState {
            robot: &direction_robot,
            current_pos: direction_robot.find_key_pos('A'),
            level,
            parent: Box::new(Some(parent.clone())),
        };
    }
    let key_state = RobotState {
        robot: &key_robot,
        current_pos: key_robot.find_key_pos('A'),
        level: 26,
        parent: Box::new(Some(parent)),
    };

    // loop through and calculate the button presses needed for each code
    let mut memo = HashMap::new();
    input.iter().map(|code| {
        let mut state = key_state.clone();
        let mut num_steps = 0;
        for next_digit in code.chars() {
            let (updated_state, cost) = path_cost(state, next_digit, &mut memo);
            num_steps += cost;
            state = updated_state;
        }
        let code_num = &code[..code.len() - 1].parse().unwrap();
        num_steps * code_num
    })
    .sum::<i64>()
}

/// The robot holds the basic behavior of our two types of robots.
/// Use a [`Vec2d`] to represent the keypad. Create a map to remember
/// the [`Point`] positions of each key, since we'll be looking those up frequently.
/// Note: this is not intended to be cloned, and deliberately does not implement it.
struct Robot {
    keypad: Vec2d<char>,
    key_positions: HashMap<char, Point>,
}

impl Robot {
    fn new(keypad: Vec2d<char>) -> Self {
        Self {
            keypad,
            key_positions: HashMap::new(),
        }
    }

    fn find_key_pos(&self, key: char) -> Point {
        if self.key_positions.contains_key(&key) {
            return self.key_positions[&key];
        }
        self.keypad.find(&key).expect("Not able to find key in keypad")
    }

    fn load_all_keys(&mut self) {
        for &key in &self.keypad.grid {
            let pos = self.find_key_pos(key);
            self.key_positions.insert(key, pos);
        }
    }
}

/// `RobotState` is a lightweight representation of where each robotic arm is at any given time.
/// This class is designed to be cloned and duplicated without bloating memory by only
/// holding onto a borrow of the [`Robot`]. So all N directional `RobotState` objects
/// hold the borrow to the same underlying [`Robot`]. We must specify a lifetime for the borrow.
#[derive(Clone)]
struct RobotState<'a> {
    robot: &'a Robot,
    current_pos: Point,
    level: i32,
    parent: Box<Option<RobotState<'a>>>,
}

impl <'a> RobotState<'a> {
    // Clone but replace the parent reference with a new reference
    fn replace_parent(&self, new_parent: RobotState<'a>) -> Self {
        Self {
            robot: self.robot,
            current_pos: self.current_pos,
            level: self.level,
            parent: Box::new(Some(new_parent)),
        }
    }
}

/// We need equals and hash, but we don't want to be comparing [`Robot`] structs, which
/// contain the keypad vector. Use level as a proxy for robot type.
impl <'a> PartialEq for RobotState<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.current_pos == other.current_pos && self.level == other.level && self.parent == other.parent
    }
}

impl <'a> Eq for RobotState<'a> {}

impl <'a> std::hash::Hash for RobotState<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current_pos.hash(state);
        self.level.hash(state);
        self.parent.hash(state);
    }
}

/// Make a node class for the min priority queue needed for pathing
#[derive(PartialEq, Eq)]
struct Node<'a> {
    state: RobotState<'a>,
    cost: i64,
}

impl <'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl <'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type MemoKey<'a> = (RobotState<'a>, char);
type MemoVal<'a> = (RobotState<'a>, i64);

/// This is where all the logic lives. Use a depth first recursion with memoization
/// dynamic programming algorithm to solve the N nested layers of robots.
/// 
/// When you have a Dijkstra's algorithm, everything looks like a pathing problem.
/// There are probably better approaches, but this one came to mind for me.
/// 
/// * To find the path from one digit to the next, use A* pathing (with manhattan distance as `h()`).
/// * The cost of moving to the next adjacent key is determined by the cost of the parent robot.
/// * This is where the recursion comes in, until we reach the top most level where the cost is 1.
/// * Memoization is essential to prevent duplicate subproblems. Robot states frequently repeat.
fn path_cost<'a>(
    robot_state: RobotState<'a>,
    destination: char,
    memo: &mut HashMap<MemoKey<'a>, MemoVal<'a>>
) -> (RobotState<'a>, i64) {
    if let Some(result) = memo.get(&(robot_state.clone(), destination)) {
        return result.clone();
    };
    if robot_state.parent.is_none() {
        // At the top level, it takes no additiona effort to press the desired button
        return (robot_state, 1);
    };

    let end = robot_state.robot.find_key_pos(destination);
    let mut queue = BinaryHeap::new();
    queue.push(Node { state: robot_state.clone(), cost: 0 });

    // Unlike Dijkstra, we don't need to keep a map of distances
    // But unlike traditional A*, we don't actually need the path, just the total cost
    let mut best_solution = (robot_state.clone(), i64::MAX);

    while let Some(current) = queue.pop() {
        let position = current.state.current_pos;
        let parent = current.state.parent.clone().unwrap();
        if current.cost + i64::from(position.manhattan_distance(&end)) > best_solution.1 {
            continue;
        }
        if position == end {
            // We've found a path to the destination
            // we don't stop, because we still have to press 'A' on parent, and a different path
            // might give us a more efficient parent cost for pressing 'A'
            let (update_parent, cost) = path_cost(parent, 'A', memo);
            let updated_state = current.state.replace_parent(update_parent);
            let final_cost = cost + current.cost;
            if final_cost < best_solution.1 {
                best_solution = (updated_state, final_cost);
            }
            continue;
        }
        for direction in [Directions::Up, Directions::Down, Directions::Left, Directions::Right] {
            let Some(next_pos) = current.state.robot.keypad.next_point(position, direction) else {
                continue;
            };
            if current.state.robot.keypad[next_pos] == 'X' {
                continue;
            };
            let parent_key = match direction {
                Directions::Up => '^',
                Directions::Down => 'v',
                Directions::Left => '<',
                Directions::Right => '>',
                _ => panic!("Invalid direction"),
            };
            let (updated_parent, parent_cost) =  path_cost(parent.clone(), parent_key, memo);
            let new_cost = current.cost + parent_cost;
            let h = new_cost + i64::from(next_pos.manhattan_distance(&end));
            if h <= best_solution.1 {
                let mut state = current.state.clone();
                state.current_pos = next_pos;
                state = state.replace_parent(updated_parent);
                let node = Node { cost: new_cost, state };
                queue.push(node);
            }
        }
    }
    memo.insert((robot_state, destination), best_solution.clone());
    best_solution
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day21::part1(&input);
        assert_eq!("126384", result.to_string())
    }
}
