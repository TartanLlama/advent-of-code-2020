fn get_seat_location(id: &str) -> i32 {
    id.chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("Invalid char"),
        })
        .fold(0, |a, b| (a << 1) + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let solution = include_str!("../input/day5.txt")
            .lines()
            .map(get_seat_location)
            .max()
            .unwrap();

        println!("Solution: {}", solution);
    }

    #[test]
    fn part2() {
        let mut ids = include_str!("../input/day5.txt")
            .lines()
            .map(get_seat_location)
            .collect::<Vec<i32>>();

        ids.sort();

        let solution = ids.windows(2).find(|w| w[0] + 1 != w[1]).unwrap()[0] + 1;

        println!("Solution: {}", solution);
    }
}
