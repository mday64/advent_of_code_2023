use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7242);
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;
    let mut energized: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut beams = vec![(0, 0, Direction::Right)];

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
            ('.', Direction::Right) | ('-', Direction::Right)=> beams.push((row, col+1, dir)),
            ('.', Direction::Left) | ('-', Direction::Left) => beams.push((row, col-1, dir)),
            ('.', Direction::Up) | ('|', Direction::Up) => beams.push((row-1, col, dir)),
            ('.', Direction::Down) | ('|', Direction::Down) => beams.push((row+1, col, dir)),
            ('-', Direction::Up) | ('-', Direction::Down) => {
                beams.push((row, col-1, Direction::Left));
                beams.push((row, col+1, Direction::Right));
            }
            ('|', Direction::Left) | ('|', Direction::Right) => {
                beams.push((row-1, col, Direction::Up));
                beams.push((row+1, col, Direction::Down));
            }
            ('/', Direction::Right) => beams.push((row-1, col, Direction::Up)),
            ('/', Direction::Left) => beams.push((row+1, col, Direction::Down)),
            ('/', Direction::Down) => beams.push((row, col-1, Direction::Left)),
            ('/', Direction::Up) => beams.push((row, col+1, Direction::Right)),
            ('\\', Direction::Right) => beams.push((row+1, col, Direction::Down)),
            ('\\', Direction::Left) => beams.push((row-1, col, Direction::Up)),
            ('\\', Direction::Down) => beams.push((row, col+1, Direction::Right)),
            ('\\', Direction::Up) => beams.push((row, col-1, Direction::Left)),
            _ => unimplemented!()
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
