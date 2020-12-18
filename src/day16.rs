use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct Rule {
    name: String,
    valid: [RangeInclusive<i64>; 2],
}

impl Rule {
    fn satisfied(&self, v: i64) -> bool {
        self.valid[0].contains(&v) || self.valid[1].contains(&v)
    }
}
struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let rule_regex = Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let rules = split.next().unwrap().lines().map(|line| {
        let captures = rule_regex.captures(line).unwrap();
        let values: Vec<i64> = [&captures[2], &captures[3], &captures[4], &captures[5]]
            .iter()
            .map(|c| c.parse().unwrap())
            .collect();
        Rule {
            name: captures[1].to_string(),
            valid: [values[0]..=values[1], values[2]..=values[3]],
        }
    });

    let parse_list = |x: &str| x.split(',').map(|i| i.parse().unwrap()).collect();
    let my_ticket: Vec<i64> = parse_list(split.next().unwrap().lines().skip(1).next().unwrap());
    let nearby_tickets: Vec<Vec<i64>> = split
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_list)
        .collect();

    Input {
        rules: rules.collect(),
        my_ticket: my_ticket,
        nearby_tickets: nearby_tickets,
    }
}

mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = parse_input(include_str!("../input/day16.txt"));
        let solution: i64 = input
            .nearby_tickets
            .iter()
            .flatten()
            .copied()
            .filter(|&val| {
                input
                    .rules
                    .iter()
                    .filter(|rule| rule.satisfied(val))
                    .count()
                    == 0
            })
            .sum();
        println!("Solution: {}", solution);
    }

    #[test]
    fn part2() {
        let input = parse_input(include_str!("../input/day16.txt"));
        let valid_tickets: Vec<_> = input
            .nearby_tickets
            .iter()
            .filter(|&ticket| {
                ticket.iter().all(|val| {
                    input
                        .rules
                        .iter()
                        .filter(|rule| rule.satisfied(*val))
                        .count()
                        > 0
                })
            })
            .collect();

        let mut rule_possibilities: HashMap<String, Vec<usize>> = input
            .rules
            .iter()
            .map(|rule| {
                let mut possibilities: Vec<_> = (0..input.my_ticket.len()).collect();

                for ticket in &valid_tickets {
                    for (i, &field) in ticket.iter().enumerate() {
                        if !rule.satisfied(field) {
                            possibilities.retain(|x| *x != i);
                        }
                    }
                }

                (rule.name.clone(), possibilities)
            })
            .collect();
            
        let mut remaining_options: Vec<_> = (0..input.my_ticket.len()).collect();
        let mut solutions = HashMap::new();
        while solutions.len() < rule_possibilities.len() {
            for (name, options) in &rule_possibilities {
                if !solutions.contains_key(&name) {
                    let filtered_options: Vec<_> = options
                        .iter()
                        .copied()
                        .filter(|option| remaining_options.contains(option))
                        .collect();
                    if filtered_options.len() == 1 {
                        remaining_options.retain(|x| *x != filtered_options[0]);
                        solutions.insert(name, filtered_options[0]);
                    }
                }
            }
        }

        let solution: i64 = solutions
            .iter()
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, &index)| input.my_ticket[index])
            .product();
        println!("Solution: {}", solution)
    }
}
