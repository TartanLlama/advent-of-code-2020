use std::collections::{BinaryHeap, HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<String, Vec<(i32, String)>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace().peekable();
            let colour = split.next().unwrap().to_owned() + split.next().unwrap();
            split.next();
            split.next();

            let &n = split.peek().unwrap();
            if n == "no" {
                return (colour, Vec::new());
            }

            let children = split
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|s| (s[0].parse().unwrap(), s[1].to_owned() + s[2]))
                .collect();

            (colour, children)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let entries = parse_input(include_str!("../input/day7.txt"));

        let mut queue = BinaryHeap::new();
        queue.push("shinygold".to_owned());

        let mut set = HashSet::new();
        while !queue.is_empty() {
            let colour = queue.pop().unwrap();
            set.insert(colour.clone());

            entries
                .iter()
                .filter(|(_, children)| children.iter().find(|(_, c)| *c == colour).is_some())
                .for_each(|(c, _)| {
                    queue.push(c.clone());
                    set.insert(c.clone());
                })
        }

        println!("Solution: {}", set.len() - 1);
    }

    #[test]
    fn part2() {
        let mut entries = parse_input(include_str!("../input/day7.txt"));

        let mut queue = BinaryHeap::new();
        queue.push(("shinygold".to_owned(), 1));

        let mut set = HashSet::new();
        let mut count = 0;
        while !queue.is_empty() {
            let (colour, n) = queue.pop().unwrap();
            set.insert(colour.clone());
            let children = &entries[&colour];
            for (i, child_colour) in children {
                count += i * n;
                queue.push((child_colour.clone(), i * n));
            }
        }

        println!("Solution: {}", count);
    }
}
