use regex::Regex;

fn validate_old_password_line(input: &str) -> bool {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(input).unwrap();
    
    let (low_bound, high_bound) = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
    let to_find = caps[3].chars().nth(0).unwrap();
    let ref password = caps[4];

    let count = password.chars()
                     .filter(|&c| c == to_find)
                     .count();

    count >= low_bound && count <= high_bound
}

fn validate_new_password_line(input: &str) -> bool {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(input).unwrap();
    
    let (first_pos, second_pos):(usize, usize) = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
    let to_find = caps[3].chars().nth(0).unwrap();
    let ref password = caps[4];

    let at_pos = |pos| password.chars()
                                          .nth(pos-1)
                                          .map_or(false, |c| c == to_find);

    at_pos(first_pos) != at_pos(second_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let valid_password_count = include_str!("../input/day2.txt")
                                       .lines()
                                       .filter(|&line| validate_old_password_line(line))
                                       .count();

        println!("Solution: {}", valid_password_count);
    }

    #[test]
    fn part2() {
        let valid_password_count = include_str!("../input/day2.txt")
                                       .lines()
                                       .filter(|&line| validate_new_password_line(line))
                                       .count();

        println!("Solution: {}", valid_password_count);
    }
}