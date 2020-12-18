use std::collections::{HashSet,HashMap};
use itertools::Itertools;

fn part1_step(active: &HashSet<(i8,i8,i8)>) -> HashSet<(i8,i8,i8)> {
    let mut counts = HashMap::new();

            for (x,y,z) in active.iter() {
                for i in -1..=1 {
                    for j in -1..=1 {
                        for k in -1..=1 {
                            if i==0 && j == 0 && k==0 {
                                continue;
                            }
                            
                    let neighbour = (x+i, y+j, z+k);
                    let count = counts.get(&neighbour).unwrap_or(&0) + 1;
                counts.insert(neighbour,count);
                        }
                    }
                }
            }
            
            counts.iter()
            .filter_map(|(cell, neighbour_count)| {
                if *neighbour_count == 3 || *neighbour_count == 2 && active.contains(cell) {
                    Some(*cell)
                } else {
                    None
                }
            })
            .collect()
}

fn part2_step(active: &HashSet<(i8,i8,i8,i8)>) -> HashSet<(i8,i8,i8,i8)> {
    let mut counts = HashMap::new();

            for (x,y,z,w) in active.iter() {
                for i in -1..=1 {
                    for j in -1..=1 {
                        for k in -1..=1 {
                            for l in -1..=1 {
                            if i==0 && j == 0 && k==0 && l==0 {
                                continue;
                            }
                            
                    let neighbour = (x+i, y+j, z+k, w+l);
                    let count = counts.get(&neighbour).unwrap_or(&0) + 1;
                counts.insert(neighbour,count);
                        }
                    }
                }
            }
        }
            
            counts.iter()
            .filter_map(|(cell, neighbour_count)| {
                if *neighbour_count == 3 || *neighbour_count == 2 && active.contains(cell) {
                    Some(*cell)
                } else {
                    None
                }
            })
            .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1(){
        let mut active = HashSet::new();
        for (i, line) in include_str!("../input/day17.txt").lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c == b'#' {
                    active.insert((i as i8,j as i8,0));
                }
            }
        }

        for _ in 0..6 {
            active = part1_step(&active);
        }

        println!("Solution: {}", active.len());
    }

    #[test]
    fn part2(){
        let mut active = HashSet::new();
        for (i, line) in include_str!("../input/day17.txt").lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c == b'#' {
                    active.insert((i as i8,j as i8,0,0));
                }
            }
        }

        for _ in 0..6 {
            active = part2_step(&active);
        }

        println!("Solution: {}", active.len());
    }
}