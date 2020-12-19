fn find_first_target_without_sum(input: &[i64]) -> i64 {
    for window in input.windows(26) {
        let target = window[25];
        match window
            .iter()
            .find(|&&a| window.iter().find(|&&b| a + b == target).is_some())
        {
            None => return target,
            _ => (),
        }
    }
    panic!("oh no");
}

fn find_range_summing_to(input: &[i64], target: i64) -> Vec<i64> {
    for start in 0..input.len() {
        for length in 2.. {
            let sum: i64 = input[start..].iter().take(length).sum();
            if sum == target {
                return input[start..]
                    .iter()
                    .take(length)
                    .copied()
                    .collect();
            }
            if sum > target {
                break;
            }
        }
    }
    panic!("oh no");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input: Vec<i64> = include_str!("../input/day9.txt")
            .lines()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>();
        let solution = find_first_target_without_sum(&input);
        println!("Solution: {}", solution);
    }

    #[test]
    fn part2() {
        let input: Vec<i64> = include_str!("../input/day9.txt")
            .lines()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>();
        let target = find_first_target_without_sum(&input);
        let solution = find_range_summing_to(&input, target);
        println!(
            "Solution: {}",
            solution.iter().min().unwrap() + solution.iter().max().unwrap()
        );
    }
}
