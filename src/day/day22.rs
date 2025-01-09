use super::Day;
use std::{collections::HashMap, fs};

/// Day 22: Monkey Market
/// 
/// We need to buy bananas from a group of monkeys in the monkey market.
/// The market price for each monkey is determined by a pseudorandom secret.
/// The secret changes based on the specified math operations (see [`next_secret`]).
/// 
/// The puzzle input is the initial secret number for each monkey.
/// 
/// Part 1: Find the 2000th secret number for each monkey, then sum them up.
/// 
/// Part 2: The price is the last digit of the secret number. Each monkey will
/// use the initial secret + the next 2000 secret numbers. The monkey will sell
/// when they see a 4 digit sequence that matches the change in price values for
/// the last 4 secrets. Example: with secret | price | change
/// ```
///      123: 3 
/// 15887950: 0 (-3)
/// 16495136: 6 (6)
///   527345: 5 (-1)
///   704524: 4 (-1)
///  1553684: 4 (0)
/// 12683156: 6 (2)
/// 11100544: 4 (-2)
/// 12249484: 4 (0)
///  7753432: 2 (-2)
/// ```
/// If you give this monkey the sequence [-1,-1,0,2] they will sell 6 bananas.
/// 
/// You can only give one sequence for all monkeys. If the sequence does not appear,
/// that monkey will not sell you any bananas. What is the maximum number of bananas
/// you can get from the monkeys?
pub struct Day22;

impl Day<Vec<i64>> for Day22 {
    fn read_input() -> Vec<i64> {
        fs::read_to_string("resources/day22.txt").expect("file day22.txt not found")
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn part1(input: &Vec<i64>) -> impl std::fmt::Display {
        input.iter()
            .map(|&initial_secret| 
                // run next secret 2000 times on the previous value
                (0..2000).fold(initial_secret, |secret, _| next_secret(secret))
            )
            .sum::<i64>()
    }

    // A little slow at 0.5 seconds on release mode, but not too bad.
    fn part2(input: &Vec<i64>) -> impl std::fmt::Display {

        // First make a map of the change sequence to the banana price for each monkey
        let price_maps = input.iter()
            .map(|&secret| build_price_map(secret))
            .collect::<Vec<_>>();

        // Once per price_map, add the price each sequence will fetch
        let mut sequence_counts = HashMap::new();
        for price_map in &price_maps {
            for key in price_map.keys() {
                *sequence_counts.entry(*key).or_insert(0) += price_map[key];
            }
        }
        let most_bananas = sequence_counts.values().max().unwrap();
        *most_bananas
    }
}

fn next_secret(secret: i64) -> i64 {
    const TRUNC: i64 = 16_777_216;
    let step1 = ((secret * 64) ^ secret) % TRUNC;
    let step2 = ((step1 / 32) ^ step1) % TRUNC;
    ((step2 * 2048) ^ step2) % TRUNC
}

fn build_price_map(secret: i64) -> HashMap<[i32; 4], i32> {
    let mut prices = Vec::new();
    let last_digit: fn(i64) -> i32 = |s| (s % 10).try_into().unwrap();

    // Add 2000 new prices in addition to the first price
    prices.push((last_digit(secret), 0));
    let mut current_secret = secret;
    for _ in 0..2000 {
        current_secret = next_secret(current_secret);
        let current_price = last_digit(current_secret);
        let last_price = prices.last().unwrap().0;
        prices.push((current_price, current_price - last_price));
    }
    
    let mut price_map= HashMap::new();
    for i in 4 .. prices.len() {
        let change_seq: [i32; 4] = [prices[i-3].1, prices[i-2].1, prices[i-1].1, prices[i].1];
        let price = prices[i].0;
        
        // the first time the sequence appears is the price for that sequence
        price_map.entry(change_seq).or_insert(price);
    }
    price_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        assert_eq!(15887950, next_secret(123));
        assert_eq!(16495136, next_secret(15887950));
        assert_eq!(527345, next_secret(16495136));
    }

    #[test]
    fn test_part_1() {
        let input = vec![1, 10, 100, 2024];
        assert_eq!("37327623", Day22::part1(&input).to_string());
    }

    #[test]
    fn test_part_2() {
        let input = vec![1, 2, 3, 2024];
        assert_eq!("23", Day22::part2(&input).to_string());
    }
}

