use super::Day;
use std::fs;
use std::collections::HashMap;

/// Day 24: Crossed Wires
/// 
/// The puzzle is a series of wires that are either 0 or 1 (false or true).
/// The puzzle also describes how the wires are connected through one of three
/// logic gates (AND, OR, XOR).
/// 
/// Part 1: Given the innitial state of wires, run through the logic gates to assign
/// values to new wires, and wait until all wires that start with the letter `z` have values.
/// All z wires are bits in a binary number with z00 being the least significant bit.
/// What is the resulting number?
/// 
/// Part 2: This is actually a program meant to add two numbers. The numbers are defined
/// by starting with `y` or starting with `x` (with y00 and x00 being the least significant bits).
/// The resulting values in the `z` wires should equal the sum of the initial numbers
/// in the x and y wires, but this is not currently working.
/// The output wires on some gates are swapped. Exactly 4 gate outputs need to be swapped back,
/// meaning that 8 output wires total need to be changed. Returng these wires as a sorted
/// comma separated string.
pub struct Day24;

type Input = (HashMap<String, bool>, Vec<Gate>);

#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    lhs: String,
    rhs: String,
    operation: Operation,
    output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Operation {
    And,
    Or,
    Xor,
}

impl Day<Input> for Day24 {
    fn read_input() -> Input {
        let input = fs::read_to_string("resources/day24.txt").expect("file day24.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Input) -> impl std::fmt::Display {
        let (wires, gates) = input;
        // run_gates wants a vector that doesn't own the Gate objects. This is important for part 2.
        // So here we need to quickly convert gates to be a Vec<&Gate> instead of Vec<Gate>
        let output = run_gates(wires, &gates.iter().collect());
        binary_num('z', &output)
    }

    /// Solved via pen/paper and debugging. The code below verifies this solution and shows
    /// some of the debugging code used to find it. The process was something like:
    /// * Observe how the logic gates build the `z` numbers. `z00` and `z01` are easy to follow.
    /// * Observing `z02`, we can notice clear patterns. Look through the each z output to find
    ///   z values that deviate from the pattern. These z values need swapped.
    /// * We can also compare the z bit outputs from what the expected sum result should be. This
    ///   tells us what z bits are wrong, and indicates roughtly where a swap is needed.
    /// * Try out the different swaps and see what works, checking against the expected result.
    fn part2(input: &Input) -> impl std::fmt::Display {
        let (wires, gates) = input;
        let mut wires = wires.clone();
        
        // change input values here for testing
        wires.entry("x16".to_string()).and_modify(|v| *v = !*v);

        // Make the swaps
        let mut gates = gates.clone();
        let swaps = vec![
            ("qff", "qnw"),
            ("z16", "pbv"),
            ("z23", "qqp"),
            ("z36", "fbq"),
        ];
        for &(s1, s2) in &swaps {
            swap_outputs(s1, s2, &mut gates);
        }

        let expected = binary_num('x', &wires) + binary_num('y', &wires);
        let wire_result = run_gates(&wires, &gates.iter().collect());
        let result = binary_num('z', &wire_result);

        // If the swaps didn't work, debug what went wrong,
        if result != expected {
            let expected_as_binary = format!("{expected:b}");
            let mut z_bits = wire_result.keys()
                .filter(|key| key.starts_with('z'))
                .collect::<Vec<_>>();
            z_bits.sort();
            z_bits.reverse();

            // find which z output wires have an unexpected value. These are potential outputs to swap.
            let wrong_zs = z_bits.into_iter().zip(expected_as_binary.chars())
                .filter(|(z_key, expected_bit)| {
                    let z_val = wire_result[*z_key];
                    (z_val && *expected_bit == '0') || (!z_val && *expected_bit == '1')
                })
                .map(|(z_key, _)| z_key)
                .map(|z_key| gates.iter().find(|gate| &gate.output == z_key).unwrap())
                .collect::<Vec<_>>();
            println!("bad zs: {:?}", wrong_zs.iter().map(|g| &g.output).collect::<Vec<_>>());
            return String::new();
        }
        
        let mut swapped = swaps.into_iter()
            .flat_map(|(s1, s2)| vec![s1, s2])
            .collect::<Vec<_>>();
        swapped.sort_unstable();
        swapped.join(",")
    }
}

/// Run the wires through the logic gates until we resolve the wire values.
/// return a new map of wire values with the result state.
fn run_gates(wires: &HashMap<String, bool>, gates: &Vec<&Gate>) -> HashMap<String, bool> {
    let mut wires = wires.clone();

    let mut unused_gates = gates.iter().collect::<Vec<_>>();
    while !unused_gates.is_empty() {
        let mut skipped = Vec::new();
        for &gate in &unused_gates {
            if !wires.contains_key(&gate.lhs) || !wires.contains_key(&gate.rhs) {
                skipped.push(gate);
                continue;
            }
            let lhs = wires[&gate.lhs];
            let rhs = wires[&gate.rhs];
            let result = match gate.operation {
                Operation::And => lhs && rhs,
                Operation::Or => lhs || rhs,
                Operation::Xor => lhs != rhs,
            };
            wires.insert(gate.output.to_string(), result);
        }

        // When swapping wires, we may create a failed solution. Kill it here
        if unused_gates == skipped {
            return wires;
        }
        unused_gates = skipped;
    }
    wires
}

fn binary_num(starting_char: char, wires: &HashMap<String, bool>) -> i64 {
    let mut bit_wires = wires.keys()
        .filter(|key| key.starts_with(starting_char))
        .collect::<Vec<_>>();
    bit_wires.sort();
    bit_wires.reverse();
    let result = bit_wires.into_iter()
        .map(|w| if wires[w] { '1' } else { '0' })
        .collect::<String>();
    i64::from_str_radix(&result, 2).unwrap()
}

// Mutating the gates in place is a little complicated, but more efficient
// and works fine for what we need it to do in part 2
fn swap_outputs(o1: &str, o2: &str, gates: &mut [Gate]) {
    let idx1 = gates.iter()
        .enumerate()
        .find(|(_, g)| g.output == o1)
        .map(|(idx, _)| idx)
        .unwrap();
    let idx2 =  gates.iter()
        .enumerate()
        .find(|(_, g)| g.output == o2)
        .map(|(idx, _)| idx)
        .unwrap();
    let g1 = gates.get_mut(idx1).unwrap();
    g1.output = o2.to_string();
    let g2 = gates.get_mut(idx2).unwrap();
    g2.output = o1.to_string();
}


fn parse_input(input: &str) -> Input {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let wires = sections[0].lines()
        .map(|line|{
            let intitial_values = line.split(": ").collect::<Vec<_>>();
            let b = intitial_values[1] == "1";
            (intitial_values[0].to_string(), b)
        })
        .collect::<HashMap<_,_>>();

    let gates = sections[1].lines()
        .map(|line|{
            let parts = line.split(" -> ").collect::<Vec<_>>();
            let gate_input = parts[0].split_whitespace().collect::<Vec<_>>();
            let operation = match gate_input[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Invalid operation {}", gate_input[1]),
            };
            Gate {
                lhs: gate_input[0].to_string(),
                rhs: gate_input[2].to_string(),
                operation,
                output: parts[1].to_string(),
            }
        })
        .collect();
    (wires, gates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result = Day24::part1(&input);
        assert_eq!("2024", result.to_string())
    }
}
