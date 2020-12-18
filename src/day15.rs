use std::collections::HashMap;

mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut tracker: HashMap<i64, usize> = "17,1,3,16,19"
            .split(",")
            .enumerate()
            .map(|(a, b)| (b.parse().unwrap(), a))
            .collect();
        let mut i = tracker.len();
        let mut old_number = 0;

        while i < (30000000-1) {
            let new_number: i64 = if tracker.contains_key(&old_number) {
                (i - tracker.get(&old_number).unwrap()) as i64
            }
            else {
                0
            };

            tracker.insert(old_number, i);
            old_number = new_number;
            i += 1;
        }

        println!("Solution: {}", old_number);
    }
}
