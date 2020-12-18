use modinverse::modinverse;

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();

    let sum: Option<i64> = residues
        .iter()
        .zip(modulii)
        .map(|(&residue, &modulus)| {
            let p = prod / modulus;
            modinverse(p, modulus).map(|inv| residue * inv * p)
        })
        .sum();

    sum.map(|s| s % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut lines = include_str!("../input/day13.txt").lines();
        let target: i32 = lines.next().unwrap().parse().unwrap();
        let (bus, time_to_wait): (i32, i32) = lines
            .next()
            .unwrap()
            .split(',')
            .filter(|&bus| bus != "x")
            .map(|bus| bus.parse().unwrap())
            .map(|bus| (bus, (target - 1) % bus))
            .max_by_key(|&(_, b)| b)
            .unwrap();
        println!(
            "Solution: {},{},{}",
            bus,
            time_to_wait,
            bus * (bus - 1 - time_to_wait)
        );
    }

    #[test]
    fn part2() {
        let mut lines = include_str!("../input/day13.txt").lines();
        let target: i32 = lines.next().unwrap().parse().unwrap();
        let ids: Vec<_> = lines.next().unwrap().split(',').collect();
        let (modulii, residues): (Vec<_>, Vec<_>) = ids
            .iter()
            .enumerate()
            .filter(|(_, &id)| id != "x")
            .map(|(i, &id)| (i as i64, id.parse::<i64>().unwrap()))
            .map(|(i, id)| (id, id-i))
            .unzip();
        let solution = chinese_remainder(&residues, &modulii).unwrap();
        println!("Solution: {}", solution);
    }
}
