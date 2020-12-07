use std::collections::HashMap;

fn parse_passport(input: &str) -> HashMap<&str, &str> {
    input.split_whitespace().map(|entry| entry.split(':')).fold(
        HashMap::new(),
        |mut acc, mut entry| {
            acc.insert(entry.next().unwrap(), entry.next().unwrap());
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let solution = include_str!("../input/day4.txt")
            .split("\n\n")
            .map(parse_passport)
            .filter(|passport| {
                required_fields
                    .iter()
                    .filter(|&&k| !passport.contains_key(k))
                    .count()
                    == 0
            })
            .count();
        println!("Solution: {}", solution);
    }

    #[test]
    fn part2() {
        let validate = |passport: &HashMap<&str, &str>| {
            match passport
                .get("byr")
                .and_then(|&s| s.parse::<i32>().ok())
                .map(|i| (1920..=2002).contains(&i))
            {
                Some(true) => (),
                _ => return false,
            };

            match passport
                .get("iyr")
                .and_then(|&s| s.parse::<i32>().ok())
                .map(|i| (2010..=2020).contains(&i))
            {
                Some(true) => (),
                _ => return false,
            };

            match passport
                .get("eyr")
                .and_then(|&s| s.parse::<i32>().ok())
                .map(|i| (2020..=2030).contains(&i))
            {
                Some(true) => (),
                _ => return false,
            };

            match passport
                .get("hgt")
                .and_then(|&s| match s.strip_suffix("cm") {
                    Some(hgt) => Some((150..=193).contains(&hgt.parse::<i32>().unwrap())),
                    _ => s
                        .strip_suffix("in")
                        .map(|hgt| (59..=76).contains(&hgt.parse::<i32>().unwrap())),
                }) {
                Some(true) => (),
                _ => return false,
            };

            match passport
                .get("hcl")
                .and_then(|hcl| hcl.strip_prefix('#'))
                .and_then(|hcl| i32::from_str_radix(hcl, 16).ok())
            {
                Some(_) => (),
                _ => return false,
            }

            match passport
                .get("ecl")
                .map(|ecl| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl))
            {
                Some(true) => (),
                _ => return false,
            }

            match passport
                .get("pid")
                .map(|pid| pid.as_bytes().len() == 9 && pid.parse::<i32>().is_ok())
            {
                Some(true) => (),
                _ => return false,
            }

            true
            //("iyr", 2010, 2020),
            //("eyr", 2020, 2030)].map(|(key, min, max) validate_int(passport, key, min, max))
        };

        let solution = include_str!("../input/day4.txt")
            .split("\n\n")
            .map(parse_passport)
            .filter(validate)
            .count();
        println!("Solution: {}", solution);
    }
}
