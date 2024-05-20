use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7242);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 7572);
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    calculate_energized(&grid, 0, 0, Direction::Right)
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;

    let max_right = (0..num_rows)
        .map(|start_row| calculate_energized(&grid, start_row, 0, Direction::Right))
        .max()
        .unwrap();
    let max_left = (0..num_rows)
        .map(|start_row| calculate_energized(&grid, start_row, num_cols-1, Direction::Left))
        .max()
        .unwrap();
    let max_down = (0..num_cols)
        .map(|start_col| calculate_energized(&grid, 0, start_col, Direction::Down))
        .max()
        .unwrap();
    let max_up = (0..num_cols)
        .map(|start_col| calculate_energized(&grid, num_rows-1, start_col, Direction::Up))
        .max()
        .unwrap();

    [max_right, max_left, max_down, max_up].into_iter().max().unwrap()
}

fn calculate_energized(
    grid: &[Vec<char>],
    start_row: isize,
    start_col: isize,
    direction: Direction,
) -> usize {
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;
    let mut energized: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut beams = vec![(start_row, start_col, direction)];

    while let Some((row, col, dir)) = beams.pop() {
        // Follow this beam until it exits the grid, or we find a
        // duplicate in `energized`
        if row < 0 || row >= num_rows || col < 0 || col >= num_cols {
            // beam has exited the grid
            continue;
        }
        if !energized.insert((row, col, dir)) {
            // beam has already been tracked here
            continue;
        }

        let ch = grid[row as usize][col as usize];
        match (ch, dir) {
            ('.', Direction::Right) | ('-', Direction::Right) => beams.push((row, col + 1, dir)),
            ('.', Direction::Left) | ('-', Direction::Left) => beams.push((row, col - 1, dir)),
            ('.', Direction::Up) | ('|', Direction::Up) => beams.push((row - 1, col, dir)),
            ('.', Direction::Down) | ('|', Direction::Down) => beams.push((row + 1, col, dir)),
            ('-', Direction::Up) | ('-', Direction::Down) => {
                beams.push((row, col - 1, Direction::Left));
                beams.push((row, col + 1, Direction::Right));
            }
            ('|', Direction::Left) | ('|', Direction::Right) => {
                beams.push((row - 1, col, Direction::Up));
                beams.push((row + 1, col, Direction::Down));
            }
            ('/', Direction::Right) => beams.push((row - 1, col, Direction::Up)),
            ('/', Direction::Left) => beams.push((row + 1, col, Direction::Down)),
            ('/', Direction::Down) => beams.push((row, col - 1, Direction::Left)),
            ('/', Direction::Up) => beams.push((row, col + 1, Direction::Right)),
            ('\\', Direction::Right) => beams.push((row + 1, col, Direction::Down)),
            ('\\', Direction::Left) => beams.push((row - 1, col, Direction::Up)),
            ('\\', Direction::Down) => beams.push((row, col + 1, Direction::Right)),
            ('\\', Direction::Up) => beams.push((row, col - 1, Direction::Left)),
            _ => unimplemented!(),
        }
    }

    // Count the number of unique locations (ignoring incoming direction)
    energized
        .into_iter()
        .map(|(row, col, _dir)| (row, col))
        .collect::<HashSet<(isize, isize)>>()
        .len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[cfg(test)]
static EXAMPLE1: &str = include_str!("../example1.txt");

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 46);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE1), 51);
}
