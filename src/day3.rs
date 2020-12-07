fn count_trees_on_slope(
    lines: &[&str], 
    (mut slope_down, mut slope_right): (usize, usize)
) -> i64 {
    let mut trees = 0;
    let (mut i, mut j) = (0,0);

    while i < lines.len() {
        if lines[i].chars().nth(j).unwrap() == '#' {
            trees += 1;
        }
        i += slope_down;
        j = (j + slope_right) % lines[0].len();
    }
    trees  
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
        let solution: i64 = [(1,1), (1,3), (1,5), (1,7), (2,1)]
                           .iter()
                           .map(|&inc| count_trees_on_slope(&lines, inc))
                           .product();
        println!("Solution: {}", solution);          
    }
}