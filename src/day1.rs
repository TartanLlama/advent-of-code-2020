use std::fs::read_to_string;
use std::path::Path;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
         .filter_map(|n| n.parse::<i32>().ok())
         .collect()
}

fn combination_summing_to_n(input: &[i32], k_combinations: usize, n: i32) -> Option<Vec<i32>> {
    input.iter()
         .copied()
         .combinations(k_combinations)
         .find(|v| v.iter().sum::<i32>() == n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn common_solution(k_combinations: usize) {
        let input = parse_input(include_str!("../input/day1.txt"));
        match combination_summing_to_n(&input, k_combinations, 2020) {
            Some(s) => println!("Solution: {}", s.iter().product::<i32>()),
            None => println!("No solution found"),
        }
    }

    #[test]
    fn part_1() {
        common_solution(2)
    }

    #[test]
    fn part_2() {
        common_solution(3)
    }
}