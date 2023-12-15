use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 107430);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 96317);
}

fn part1(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.tilt_north();
    puzzle.total_load()
}

//
// It's clear that with 1,000,000,000 iterations, we'll need to find a
// repeating pattern.  The question is whether we need to check the
// locations of all 'O' characters, or just the load value.  I'm going
// to guess and say the load value is sufficient.
//
// It turns out that load value alone is insufficient.  I guess I'll
// go with the raw state as a big String.
//
fn part2(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);

    // key is state (as a String), value is (iteration seen, load value)
    let mut history = HashMap::<String, (usize, usize)>::new();

    for i in 0..1_000_000_000 {
        // Do the North, West, South, East tilts
        puzzle.tilt_north();
        puzzle.tilt_west();
        puzzle.tilt_south();
        puzzle.tilt_east();

        // Get the current state and load value
        let state = puzzle.to_string();
        let load_value = puzzle.total_load();

        // println!("Iteration {i}: load_value={load_value}");

        // See if we've seen this load value before
        if let Some((prior, _load)) = history.get(&state) {
            let cycle_length = i - prior;
            let remainder = (999_999_999 - i) % cycle_length;
            // Return the key for value `prior + remainder`
            return history.iter().find(|(_k,(i,_l))| i == &(prior + remainder)).unwrap().1.1;
        } else {
            history.insert(state, (i, load_value));
        }
    }
    
    panic!("No cycle found!");
}

struct Puzzle {
    grid: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

impl Puzzle {
    fn from_str(input: &str) -> Puzzle {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        
        Puzzle { grid, num_rows, num_cols }
    }

    fn to_string(&self) -> String {
        self.grid.iter().map(|v| v.iter().join("")).join("\n")
    }

    fn tilt_north(&mut self) {
        for row in 1..self.num_rows {
            for col in 0..self.num_cols {
                // If this row/column has an 'O', try to shift it up as
                // much as possible.
                if self.grid[row][col] == 'O' {
                    let mut r = row;
                    while r > 0 && self.grid[r-1][col] == '.' {
                        r -= 1;
                    }
                    if r != row {
                        self.grid[row][col] = '.';
                        self.grid[r][col] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for row in (0..self.num_rows-1).rev() {
            for col in 0..self.num_cols {
                // If this row/column has an 'O', try to shift it up as
                // much as possible.
                if self.grid[row][col] == 'O' {
                    let mut r = row;
                    while r < self.num_rows-1 && self.grid[r+1][col] == '.' {
                        r += 1;
                    }
                    if r != row {
                        self.grid[row][col] = '.';
                        self.grid[r][col] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for col in 1..self.num_cols {
            for row in 0..self.num_rows {
                // If this row/column has an 'O', try to shift it up as
                // much as possible.
                if self.grid[row][col] == 'O' {
                    let mut c = col;
                    while c > 0 && self.grid[row][c-1] == '.' {
                        c -= 1;
                    }
                    if c != col {
                        self.grid[row][col] = '.';
                        self.grid[row][c] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for col in (0..self.num_cols-1).rev() {
            for row in 0..self.num_rows {
                // If this row/column has an 'O', try to shift it up as
                // much as possible.
                if self.grid[row][col] == 'O' {
                    let mut c = col;
                    while c < self.num_cols-1 && self.grid[row][c+1] == '.' {
                        c += 1;
                    }
                    if c != col {
                        self.grid[row][col] = '.';
                        self.grid[row][c] = 'O';
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        self.grid.iter().enumerate().map(|(i,row)| {
            let load_multiplier = self.num_rows - i;
            row.iter().filter(|c| c==&&'O').count() * load_multiplier
        })
        .sum()
    }
}

#[cfg(test)]
static EXAMPLE1: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 136);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE1), 64);
}
