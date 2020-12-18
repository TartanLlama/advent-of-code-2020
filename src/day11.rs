#[derive(PartialEq)]
enum Cell {
    Occupied,
    Empty,
    Floor,
}

fn run_state_part1(state: &mut Vec<Vec<Cell>>) -> bool {
    let mut adjacent = vec![vec![0; state[0].len()]; state.len()];

    for (i, line) in state.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            let ii = i as i64;
            let ij = j as i64;
            let occupied_neighbours = [
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 1),
                (-1, 1),
                (1, 0),
                (-1, 0),
                (-1, -1),
            ]
            .iter()
            .filter(|(a, b)| {
                a + ii >= 0
                    && a + ii < state.len() as i64
                    && b + ij >= 0
                    && b + ij < state[0].len() as i64
            })
            .filter(|(a, b)| state[(ii + a) as usize][(ij + b) as usize] == Cell::Occupied)
            .count();
            adjacent[i][j] = occupied_neighbours;
        }
    }

    let mut changed = false;

    for (i, line) in state.iter_mut().enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            if adjacent[i][j] == 0 && *cell == Cell::Empty {
                *cell = Cell::Occupied;
                changed = true;
            } else if adjacent[i][j] >= 4 && *cell == Cell::Occupied {
                *cell = Cell::Empty;
                changed = true;
            }
        }
    }

    changed
}

fn run_state_part2(state: &mut Vec<Vec<Cell>>) -> bool {
    let mut adjacent = vec![vec![0; state[0].len()]; state.len()];

    for (i, line) in state.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            let ii = i as i64;
            let ij = j as i64;
            let occupied_neighbours = [
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 1),
                (-1, 1),
                (1, 0),
                (-1, 0),
                (-1, -1),
            ]
            .iter()
            .filter(|(a, b)| {
                let (mut x, mut y) = (ii, ij);
                while (0..state.len() as i64).contains(&(x + a)) && (0..state[0].len() as i64).contains(&(y + b)) {
                    x += a;
                    y += b;
                    if state[x as usize][y as usize] == Cell::Occupied {
                        return true;
                    }
                    if state[x as usize][y as usize] == Cell::Empty {
                        return false;
                    }
                }
                false
            })
            .count();
            adjacent[i][j] = occupied_neighbours;
        }
    }

    let mut changed = false;

    for (i, line) in state.iter_mut().enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            if adjacent[i][j] == 0 && *cell == Cell::Empty {
                *cell = Cell::Occupied;
                changed = true;
            } else if adjacent[i][j] >= 5 && *cell == Cell::Occupied {
                *cell = Cell::Empty;
                changed = true;
            }
        }
    }

    changed
}
#[cfg(test)]
mod tests {
    use super::*;

    fn common(f: fn(&mut Vec<Vec<Cell>>) -> bool) {
        let mut state: Vec<Vec<Cell>> = include_str!("../input/day11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Occupied,
                    'L' => Cell::Empty,
                    '.' => Cell::Floor,
                    _ => panic!("bad char"),
                })
                .collect()
        })
        .collect();

    while f(&mut state) {}
    let solution: usize = state
        .iter()
        .map(|l| l.iter().filter(|&c| *c == Cell::Occupied).count())
        .sum();
    println!("Solution: {}", solution);
    }

    #[test]
    fn part1() {
        common(run_state_part1);
    }

    #[test]
    fn part2() {
        common(run_state_part2);
    }
}
