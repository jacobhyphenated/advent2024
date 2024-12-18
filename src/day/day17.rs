use super::Day;
use std::fs;

/// Day 17: Chronospatial Computer
/// 
/// The puzzle input represents a computer that uses 3 bit instructions.
/// The computer has 3 registers (a, b, c) that can hold any arbitrarily large number.
/// the program is a list of 3 bit numbers [0-7] with the first number representing the operator
/// and the second representing the operand.
/// 
/// Each operation has a set of defined rules on what gets executed and how the operand is used.
/// See AOC for the complete rule list definition, or [`run_program`] for the implementation.
/// 
/// Part 1: Run the program and enter the output as a comma separated list of integers.
/// 
/// Part 2: The program is supposed to output itself. What is the lowest value to start
/// in register a that would cause the program to output itself?
pub struct Day17;

#[derive(Debug, Clone)]
pub struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    output: Vec<u64>,
}

type Debugger = (Computer, Vec<u64>);

impl Day<Debugger> for Day17 {
    fn read_input() -> Debugger {
        let input = fs::read_to_string("resources/day17.txt").expect("file day17.txt not found");
        parse_input(&input)
    }

    // Straightforware implementation of the program logic and running it.
    fn part1(input: &Debugger) -> impl std::fmt::Display {
        let (computer, program) = input;
        let mut computer = computer.clone();
        run_program(&mut computer, program);

        // Rust has a `join()` but it only works on strings, not u64
        let output = computer.output.iter()
            .fold("".to_string(), |acc, next| format!("{acc},{next}" ));
        output[1..].to_string()
    }

    /// This requires some explanation.
    /// Started with Pen and Paper to work out how the program executes and what it does.
    /// Register a is divided by 8 in each execution pass until the value is 0 at the end.
    /// 
    /// Determine the starting a value by working backward.
    /// 1. To get 0 a the end, the last a value would need to be between 0 and 7 ( `a / 8 = 0` with truncation).
    /// 2. Run the program with [0-7] in the a register and see which value outputs the correct result
    ///    for the last digit in the program code.
    /// 3. Now take this "success" value and multiply by 8. Except that's not sufficient (again, truncation).
    /// 4. So take [success * 8, success * 8 + 8). This range represents all possible states that end in success.
    /// 5. Now run the program and compare the output (now 2 digits) to the last 2 digits of the program.
    /// 6. Repeat this process until we solve for the full length of the program
    fn part2(input: &Debugger) -> impl std::fmt::Display {
        let (computer, program) = input;
        let mut possible_values = vec![0];
        let mut from_end = program.len();
        while from_end > 0 {
            from_end -= 1;
            possible_values = possible_values.into_iter()
                .flat_map(|a| a * 8 .. a * 8 + 8)
                .map(|a| {
                    let mut test_computer = computer.clone();
                    test_computer.register_a = a;
                    run_program(&mut test_computer, program);
                    (a, test_computer.output.to_owned())
                })
                .filter(|(_, output)| output[..] == program[from_end..])
                .map(|(a, _)| a)
                .collect();
        }
        possible_values.into_iter().min().unwrap()
    }
}

fn run_program(computer: &mut Computer, program: &Vec<u64>) {
    let mut instruction_pointer = 0;
    while let Some(&operator) = program.get(instruction_pointer) {
        let operand = program[instruction_pointer + 1];
        match operator {
            0 => computer.register_a /= u64::pow(2, computer.combo_operand(operand).try_into().unwrap()),
            1 => computer.register_b ^= operand,
            2 => computer.register_b = computer.combo_operand(operand) % 8,
            3 => if computer.register_a != 0 { instruction_pointer = operand as usize },
            4 => computer.register_b ^= computer.register_c,
            5 => computer.output.push(computer.combo_operand(operand) % 8),
            // the rust exponential methods for u64 take a u64 and a u32. Some lossy casting must be performed
            6 => computer.register_b = computer.register_a / u64::pow(2, computer.combo_operand(operand).try_into().unwrap()),
            7 => computer.register_c = computer.register_a / u64::pow(2, computer.combo_operand(operand).try_into().unwrap()),
            _ => println!("Invalid operand {operand}"),
        }

        if operator != 3 || computer.register_a == 0 {
            instruction_pointer += 2;
        }
    } 
}

impl Computer {
    fn combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0 ..= 3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("operand {operand} is reserved and not implemented"),
        }
    }
}

fn parse_input(input: &str) -> Debugger {
    let lines = input.lines().collect::<Vec<_>>();
    let parse_register = |line: &str| line.split(": ").last().unwrap().trim().parse::<u64>().unwrap();
    let register_a = parse_register(lines[0]);
    let register_b = parse_register(lines[1]);
    let register_c = parse_register(lines[2]);

    let program = lines[4].split(": ").last().unwrap().trim()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let computer = Computer { register_a, register_b, register_c, output: Vec::new() };
    (computer, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0";
        let input = parse_input(test_input);
        let result =  Day17::part1(&input);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let test_input = "Register A: 2024
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0";
        let input = parse_input(test_input);
        let result =  Day17::part2(&input);
        assert_eq!("117440", result.to_string())
    }

}
