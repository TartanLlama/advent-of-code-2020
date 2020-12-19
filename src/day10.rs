#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let mut adapters: Vec<i32> = include_str!("../input/day10.txt")
            .lines()
            .map(|a| a.parse().unwrap())
            .collect();
        adapters.push(0);
        adapters.sort();
        let diffs: Vec<i32> = adapters
            .windows(2)
            .map(|p| p[1] - p[0])
            .take_while(|&diff| diff <= 3)
            .collect();
        let one_diffs = diffs.iter().filter(|&&d| d == 1).count();
        let three_diffs = diffs.iter().filter(|&&d| d == 3).count() + 1;
        println!("Solution: {}", one_diffs * three_diffs);
    }

    #[test]
    fn part2() {
        let mut adapters: Vec<i32> = include_str!("../input/day10.txt")
            .lines()
            .map(|a| a.parse().unwrap())
            .collect();
        adapters.push(0);
        adapters.push(adapters.iter().max().unwrap() + 3);
        adapters.sort();

        let mut branch_counts: Vec<i64> = Vec::new();
        branch_counts.resize(adapters.len(), 0);
        branch_counts[adapters.len() - 1] = 1;

        for i in (0..adapters.len()).rev() {
            branch_counts[i] = adapters[i..]
                .iter()
                .enumerate()
                .take_while(|(_, &other)| other - adapters[i] <= 3)
                .map(|(index, _)| branch_counts[index+i])
                .sum();
        }

        println!("Solution: {}", branch_counts[0]);
    }
}
