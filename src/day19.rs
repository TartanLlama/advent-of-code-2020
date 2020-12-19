use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Production {
    Terminal(char),
    NonTerminal(usize, usize),
}

struct Grammar {
    productions: Vec<(usize, Production)>,
}

impl Grammar {
    fn unit_productions(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.productions.iter().filter_map(|&(k, v)| match v {
            Production::Terminal(c) => Some((k, c)),
            _ => None,
        })
    }

    fn non_terminal_productions(&self) -> impl Iterator<Item = (usize, (usize, usize))> + '_ {
        self.productions.iter().filter_map(|&(k, v)| match v {
            Production::NonTerminal(a, b) => Some((k, (a, b))),
            _ => None,
        })
    }

    fn recognise(&self, input: &str) -> bool {
        //substring_map[a][b][c] stores whether a substring of size a+1
        //starting at index b is recognised by production rule c
        let mut substring_map =
            vec![vec![vec![false; self.productions.len()]; input.len()]; input.len()];

        //Initialize with all unit productions    
        for substring_start in 0..input.len() {
            for (rule, c) in self.unit_productions() {
                if input.chars().nth(substring_start).unwrap() == c {
                    substring_map[0][substring_start][rule] = true;
                }
            }
        }
    
        //Loop through all possible substrings, determining which rules
        //recognise that substring
        for substring_length in 2..=input.len() {
            for substring_start in 0..=input.len() - substring_length {
                for partition in 1..substring_length {
                    for (rule_from, (rule_to_a, rule_to_b)) in self.non_terminal_productions() {
                        if substring_map[partition-1][substring_start][rule_to_a]
                            && substring_map[substring_length - partition-1][substring_start + partition][rule_to_b]
                        {
                            substring_map[substring_length-1][substring_start][rule_from] = true;
                        }
                    }
                }
            }
        }
    
        //return if the whole string is accepted by rule 0
        substring_map[input.len() - 1][0][0]
    }
}

fn parse_grammar(input: &str) -> Grammar {
    let terminal_re = Regex::new(r#"(\d+): "(.)""#).unwrap();
    let nonterminal_re = Regex::new(r"(\d+): (\d+) (\d+)(?: \| (\d+) (\d+))?").unwrap();
    let productions = input
        .lines()
        .map(|line| {
            if let Some(caps) = terminal_re.captures(line) {
                vec![(
                    caps[1].parse().unwrap(),
                    Production::Terminal(caps[2].chars().nth(0).unwrap()),
                )]
            } else if let Some(caps) = nonterminal_re.captures(line) {
                if let Some(_) = caps.get(4) {
                    vec![
                        (
                            caps[1].parse().unwrap(),
                            Production::NonTerminal(
                                caps[2].parse().unwrap(),
                                caps[3].parse().unwrap(),
                            ),
                        ),
                        (
                            caps[1].parse().unwrap(),
                            Production::NonTerminal(
                                caps[4].parse().unwrap(),
                                caps[5].parse().unwrap(),
                            ),
                        ),
                    ]
                } else {
                    vec![(
                        caps[1].parse().unwrap(),
                        Production::NonTerminal(caps[2].parse().unwrap(), caps[3].parse().unwrap()),
                    )]
                }
            } else {
                panic!("Unrecognised production: {}", line);
            }
        })
        .flatten()
        .collect();
    Grammar {
        productions: productions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut split = include_str!("../input/day19.txt").split("\n\n");
        let grammar = parse_grammar(split.next().unwrap());
        
        let solution = split
            .next()
            .unwrap()
            .lines()
            .filter(|input| cyk(&grammar, &input))
            .count();
        println!("Solution: {}", solution);
    }
}
