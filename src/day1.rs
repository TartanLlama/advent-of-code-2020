use std::fs::read_to_string;
use std::path::Path;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
         .filter_map(|n| n.parse::<i32>().ok())
         .collect()
}

fn combination_summing_to_n(input: &[i32], k_combinations: usize, n: i32) -> Option<(i32,i32)> {
    input.iter()
         .enumerate()
         .flat_map(|(i,&a)| input.iter().skip(i).map(move |&b| (a,b)))
         .find(|(a,b)| a+b == n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn common_solution(k_combinations: usize) {
        let input = parse_input(include_str!("../input/day1.txt"));
        match combination_summing_to_n(&input, k_combinations, 2020) {
            Some((a,b)) => println!("Solution: {}", a*b),
            None => println!("No solution found"),
        }
    }

    #[test]
    fn part_1() {
        common_solution(2)
    }


}