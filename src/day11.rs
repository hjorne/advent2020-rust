#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type Grid = Vec<Vec<State>>;

pub fn day11() {
    let grid = parse();

    let ans1 = run_sim(&grid, 4, count_neighbors);
    assert_eq!(ans1, 2303);

    let ans2 = run_sim(&grid, 5, count_los);
    assert_eq!(ans2, 2057);
}

fn run_sim(grid: &Grid, limit: i64, counter: fn(i64, i64, &Grid) -> i64) -> usize {
    let mut prev = grid.clone();
    let mut next = step(&prev, limit, counter);

    while prev != next {
        let tmp = step(&next, limit, counter);
        prev = next;
        next = tmp;
    }

    count_occ(&next)
}

fn count_occ(grid: &Grid) -> usize {
    grid.into_iter()
        .map(|row| row.into_iter().filter(|&&el| el == State::Occupied).count())
        .sum()
}

fn step(grid: &Grid, limit: i64, counter: fn(i64, i64, &Grid) -> i64) -> Grid {
    let mut new_grid = grid.clone();
    for x in 0..grid.len() {
        for y in 0..grid[x as usize].len() {
            let occ = counter(x as i64, y as i64, &grid);
            match grid[x][y] {
                State::Floor => {}
                State::Empty => {
                    if occ == 0 {
                        new_grid[x][y] = State::Occupied
                    }
                }
                State::Occupied => {
                    if occ >= limit {
                        new_grid[x][y] = State::Empty
                    }
                }
            }
        }
    }
    new_grid
}

fn count_neighbors(x: i64, y: i64, grid: &Grid) -> i64 {
    let mut occ = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let x_new = x + dx;
            let y_new = y + dy;
            if x_new >= 0
                && x_new < grid.len() as i64
                && y_new >= 0
                && y_new < grid[x as usize].len() as i64
                && dx.abs() + dy.abs() != 0
            {
                match grid[x_new as usize][y_new as usize] {
                    State::Floor => {}
                    State::Empty => {}
                    State::Occupied => occ += 1,
                }
            }
        }
    }
    occ
}

fn count_los(x: i64, y: i64, grid: &Grid) -> i64 {
    let mut occ = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let mut x_new = x + dx;
            let mut y_new = y + dy;
            while x_new >= 0
                && x_new < grid.len() as i64
                && y_new >= 0
                && y_new < grid[x as usize].len() as i64
                && dx.abs() + dy.abs() != 0
            {
                match grid[x_new as usize][y_new as usize] {
                    State::Floor => {}
                    State::Empty => {
                        break;
                    }
                    State::Occupied => {
                        occ += 1;
                        break;
                    }
                }

                x_new = x_new + dx;
                y_new = y_new + dy;
            }
        }
    }
    occ
}

fn parse() -> Grid {
    std::fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    '#' => State::Occupied,
                    'L' => State::Empty,
                    _ => panic!("Unknown character"),
                })
                .collect()
        })
        .collect()
}

#[test]
fn day11_test() {
    day11();
}
