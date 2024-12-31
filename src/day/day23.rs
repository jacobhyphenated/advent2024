use super::Day;
use std::collections::{HashMap, HashSet};
use std::fs;

pub struct Day23;

type Network = HashMap<String, HashSet<String>>;

/// Day 23: LAN Party
/// 
/// Computers are connected to each other as described in the puzzle input.
/// The connections are bi-directional.
/// 
/// Part 1: Find sets of 3 interconnected computers such that all 3 computers are connected to each other.
/// Consider only computers that start with the letter `t`. How many such sets
/// of three interconnected computers are there?
/// 
/// Part 2: The LAN will be a sub network where every computer has a connection to all the others.
/// Find the largest such sub network, then display each computer name alphabetically (comma separated).
impl Day<Network> for Day23 {
    fn read_input() -> Network {
        let input = fs::read_to_string("resources/day23.txt").expect("file day23.txt not found");
        parse_input(&input)
    }

    // Brute force part 1, which is fairly easy considering 3 node sets
    fn part1(input: &Network) -> impl std::fmt::Display {
        let mut three_set: HashSet<Vec<&String>> = HashSet::new();
        for t_key in input.keys().filter(|s| s.starts_with('t')) {
            for second_node in &input[t_key] {
                for third_node in &input[second_node] {
                    if !input[t_key].contains(third_node) {
                        continue;
                    }
                    // sort my list of 3 network nodes to prevent duplicates
                    // can't use a set for this because Rust `HashSet` does not implement Hash
                    let mut set = vec![t_key, second_node, third_node];
                    set.sort();
                    three_set.insert(set);
                }
            }
        }
        three_set.len()
    }

    // Sovle using the Bron Kerbosch algorithm
    fn part2(input: &Network) -> impl std::fmt::Display {
        let mut results = Vec::new();
        let keys = input.keys().map(|k| k.as_str()).collect::<HashSet<_>>();
        bron_kerbosch(
            HashSet::new(),
            keys,
            HashSet::new(),
            input,
            &mut results,
        );
        let largest_clique = results
            .into_iter()
            .max_by(|r1, r2| r1.len().cmp(&r2.len()))
            .unwrap();
        let mut result = largest_clique.into_iter().collect::<Vec<_>>();
        result.sort();
        result.join(",")
    }
}

/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
/// 
/// Bron Kerbosch finds the maximum cliques of a graph using recursive backtracking.
/// This variant calculates a 'pivot' point to reduce the number of recursive calls.
/// the pivot is chosen as the vertix with a large number of edges.
/// 
/// Rust note: String vs. &String vs. &str vs. &&str
/// When doing set unions/interesections on sets of `&str`, the resulting iterator has `&&str`.
/// This doesn't work for our purposes, but `.copied()` calls copy, which is a copy of the `&str`
/// pointer and not the underlying string, converting our `&&str` to `&str`. 
fn bron_kerbosch<'a>(
    clique: HashSet<&'a str>,
    mut vertices: HashSet<&'a str>,
    mut exclusion: HashSet<&'a str>,
    network: &'a Network,
    results: &mut Vec<HashSet<&'a str>>,
) {
    if vertices.is_empty() {
        if exclusion.is_empty() {
            results.push(clique.clone());
        }
        return;
    }

    let mut pivot_keys = vertices.union(&exclusion).into_iter().collect::<Vec<_>>();
    pivot_keys.sort_by(|&&k1, &&k2| network[k2].len().cmp(&network[k1].len()));
    let pivot = pivot_keys[0];
    let pivot_neighbors = neighbors(*&pivot, network);
    let sub_graph_vertices = vertices.difference(&pivot_neighbors)
        .copied()
        .collect::<HashSet<_>>();
    for v in sub_graph_vertices {
        let v_set = [v].into_iter().collect::<HashSet<_>>();
        let v_neighbors = neighbors(v, network);
        bron_kerbosch(
            clique.union(&v_set).copied().collect(),
            vertices.intersection(&v_neighbors).copied().collect(),
            exclusion.intersection(&v_neighbors).copied().collect(),
            network,
            results,
        );
        vertices = vertices.difference(&v_set).copied().collect();
        exclusion = exclusion.union(&v_set).copied().collect();
    }
}

/// One of the hardest parts of this problem was rust `String` vs `&str` stuff.
/// This helper method gets the nodes connected to the `v` parameter, but 
/// converts the &String references to `&str` for use in the main function call.
fn neighbors<'a>(v: &'a str, network: &'a Network) -> HashSet<&'a str> {
    network[v]
        .iter()
        .map(|s| s.as_str())
        .collect::<HashSet<_>>()
}

fn parse_input(input: &str) -> Network {
    let mut network = HashMap::new();
    for connection in input.lines() {
        let parts = connection.split('-').collect::<Vec<_>>();
        let lhs = parts[0].to_string();
        let rhs = parts[1].to_string();

        network
            .entry(lhs.clone())
            .or_insert_with(|| HashSet::new())
            .insert(rhs.clone());
        network
            .entry(rhs)
            .or_insert_with(|| HashSet::new())
            .insert(lhs);
    }

    network
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result = Day23::part1(&input);
        assert_eq!("7", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result = Day23::part2(&input);
        assert_eq!("co,de,ka,ta", result.to_string())
    }
}
