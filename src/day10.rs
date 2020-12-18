/// A function type that takes its own type as an input is an infinite recursive type.
/// We introduce the "Apply" trait, which will allow us to have an input with the same type as self, and break the recursion.
/// The input is going to be a trait object that implements the desired function in the interface.
trait Apply<T, R> {
    fn apply(&self, f: &dyn Apply<T, R>, t: T) -> R;
}

/// If we were to pass in self as f, we get:
/// λf.λt.sft
/// => λs.λt.sst [s/f]
/// => λs.ss
impl<T, R, F> Apply<T, R> for F
where
    F: Fn(&dyn Apply<T, R>, T) -> R,
{
    fn apply(&self, f: &dyn Apply<T, R>, t: T) -> R {
        self(f, t)
    }
}

/// (λt(λx.(λy.xxy))(λx.(λy.f(λz.xxz)y)))t
/// => (λx.xx)(λx.f(xx))
/// => Yf
fn y<T, R>(f: impl Fn(&dyn Fn(T) -> R, T) -> R) -> impl Fn(T) -> R {
    move |t| {
        (&|x: &dyn Apply<T, R>, y| x.apply(x, y))(
            &|x: &dyn Apply<T, R>, y| f(&|z| x.apply(x, z), y),
            t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut adapters: Vec<i32> = include_str!("../input/day10.txt")
            .lines()
            .map(|a| a.parse().unwrap())
            .collect();
        adapters.push(0);
        adapters.sort();
        let diffs: Vec<i32> = adapters
            .windows(2)
            .map(|p| p[1] - p[0])
            .take_while(|&diff| diff <= 3)
            .collect();
        let one_diffs = diffs.iter().filter(|&&d| d == 1).count();
        let three_diffs = diffs.iter().filter(|&&d| d == 3).count() + 1;
        println!("Solution: {}", one_diffs * three_diffs);
    }

    #[test]
    fn part2() {
        let mut adapters: Vec<i32> = include_str!("../input/day10.txt")
            .lines()
            .map(|a| a.parse().unwrap())
            .collect();
        adapters.push(0);
        adapters.push(adapters.iter().max().unwrap() + 3);
        adapters.sort();

        let mut branch_counts: Vec<i64> = Vec::new();
        branch_counts.resize(adapters.len(), 0);
        branch_counts[adapters.len() - 1] = 1;

        for i in (0..adapters.len()).rev() {
            branch_counts[i] = adapters[i..]
                .iter()
                .enumerate()
                .take_while(|(_, &other)| other - adapters[i] <= 3)
                .map(|(index, _)| branch_counts[index+i])
                .sum();
        }

        println!("Solution: {}", branch_counts[0]);
    }
}
