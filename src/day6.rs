use alga::general::*;

#[derive(Clone, Copy)]
struct BitAnd;
impl Operator for BitAnd {
    fn operator_token() -> Self {
        BitAnd
    }
}
#[derive(Clone, Copy)]
struct BitOr;
impl Operator for BitOr {
    fn operator_token() -> Self {
        BitOr
    }
}

impl Identity<BitAnd> for u32 {
    fn identity() -> u32 {
        u32::max_value()
    }
}
impl Identity<BitOr> for u32 {
    fn identity() -> u32 {
        0
    }
}

impl AbstractMagma<BitAnd> for u32 {
    fn operate(&self, lhs: &Self) -> u32 {
        self & lhs
    }
}
impl AbstractMagma<BitOr> for u32 {
    fn operate(&self, lhs: &Self) -> u32 {
        self | lhs
    }
}

impl AbstractSemigroup<BitAnd> for u32 {}
impl AbstractSemigroup<BitOr> for u32 {}
impl AbstractMonoid<BitAnd> for u32 {}
impl AbstractMonoid<BitOr> for u32 {}

fn count_answers<Op: Operator>(input: &str) -> u32
where
    u32: AbstractMonoid<Op>,
{
    input
        .split("\n")
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| (c as u32) - ('a' as u32))
                .fold(0, |acc, i| acc | (1u32 << (i + 1)))
        })
        .fold(u32::identity(), |acc, i| acc.operate(&i))
        .count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let solution: u32 = include_str!("../input/day6.txt")
            .split("\n\n")
            .map(count_answers::<BitOr>)
            .sum();

        println!("Solution: {}", solution);
    }

    #[test]
    fn part2() {
        let solution: u32 = include_str!("../input/day6.txt")
            .split("\n\n")
            .map(count_answers::<BitAnd>)
            .sum();

        println!("Solution: {}", solution);
    }
}
