use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut zero_mask = u64::max_value();
        let mut one_mask = 0u64;

        let bitmask_regex = Regex::new(r"mask = (.+)").unwrap();
        let memset_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

        for line in include_str!("../input/day14.txt").lines() {
            if let Some(mask) = bitmask_regex.captures(line) {
                zero_mask = u64::from_str_radix(&mask[1].replace("X", "1"), 2).unwrap();
                one_mask = u64::from_str_radix(&mask[1].replace("X", "0"), 2).unwrap();
            } else if let Some(mem) = memset_regex.captures(line) {
                let addr = mem[1].parse().unwrap();
                let val = mem[2].parse::<u64>().unwrap() & zero_mask | one_mask;
                memory.insert(addr, val);
            }
        }

        let solution = memory.iter().fold(0, |acc, (_, val)| acc + val);
        println!("Solution: {}", solution);
    }
    #[test]
    fn part2() {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut x_positions = Vec::new();
        let mut one_mask = 0u64;

        let bitmask_regex = Regex::new(r"mask = (.+)").unwrap();
        let memset_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

        for line in include_str!("../input/day14.txt").lines() {
            if let Some(mask) = bitmask_regex.captures(line) {
                x_positions = mask[1]
                    .bytes()
                    .positions(|c| c == b'X')
                    .map(|i| 36 - i - 1)
                    .collect();
                one_mask = u64::from_str_radix(&mask[1].replace("X", "0"), 2).unwrap();
            } else if let Some(mem) = memset_regex.captures(line) {
                let addr: u64 = mem[1].parse().unwrap();
                let val = mem[2].parse::<u64>().unwrap();
                for bits in
                    itertools::repeat_n([0, 1].iter(), x_positions.len()).multi_cartesian_product()
                {
                    let addr_masked = bits.iter().enumerate().fold(addr, |acc, (i, &&bit)| {
                        if bit == 1 {
                            acc | (1 << x_positions[i])
                        } else {
                            acc & !(1 << x_positions[i])
                        }
                    }) | one_mask;

                    memory.insert(addr_masked, val);
                }
            }
        }

        let solution = memory.iter().fold(0, |acc, (_, val)| acc + val);
        println!("Solution: {}", solution);
    }
}
