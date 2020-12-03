#[derive(Eq, PartialEq)]
enum Tile {
    Tree,
    Space,
}

type Grid = Vec<Vec<Tile>>;

pub fn day3() {
    let grid = parse();

    let ans1 = count(&grid, 3, 1);
    assert_eq!(ans1, 237);

    let ans2 = part2(&grid);
    assert_eq!(ans2, 2106818610);
}

fn count(grid: &Grid, x_step: usize, y_step: usize) -> usize {
    let (n, m) = (grid.len(), grid[0].len());
    let (mut x, mut y) = (0, 0);
    let mut count = 0;

    while y < n {
        if grid[y][x] == Tile::Tree {
            count += 1;
        }
        x = (x + x_step) % m;
        y = y + y_step;
    }

    count
}

fn part2(grid: &Grid) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x_step, y_step)| count(grid, x_step, y_step))
        .fold(1, |x, y| x * y)
}

fn parse() -> Grid {
    std::fs::read_to_string("inputs/day3.txt")
        .unwrap()
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Tree,
                    _ => panic!("Unknown character {}", c),
                })
                .collect()
        })
        .collect()
}

#[test]
fn day3_test() {
    day3();
}
