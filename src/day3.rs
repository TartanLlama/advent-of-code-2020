fn count_trees_on_slope(
    lines: &[&str], 
    (mut slope_down, mut slope_right): (usize, usize)
) -> i64 {
    let steps = (0..)
                                    .step_by(slope_right)
                                    .map(|i| i % lines[0].len());

    lines.iter()
         .step_by(slope_down)
         .zip(steps)
         .map(|(line, i)| (line.chars().nth(i).unwrap() == '#') as i64)
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines: Vec<&str> = include_str!("../input/day3.txt").lines().collect();
        let trees = count_trees_on_slope(&lines, (1,3));
        println!("Solution: {}", trees);          
    }

    #[test]
    fn part2() {
        let lines: Vec<&str> = include_str!("../input/day3.txt").lines().collect();
        let solution: i64 = vec![(1,1), (1,3), (1,5), (1,7), (2,1)]
                           .iter()
                           .map(|&inc| count_trees_on_slope(&lines, inc))
                           .product();
        println!("Solution: {}", solution);          
    }
}