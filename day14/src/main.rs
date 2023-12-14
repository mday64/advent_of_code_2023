fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 107430);
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for row in 1..num_rows {
        for col in 0..num_cols {
            // If this row/column has an 'O', try to shift it up as
            // much as possible.
            if grid[row][col] == 'O' {
                let mut r = row;
                while r > 0 && grid[r-1][col] == '.' {
                    r -= 1;
                }
                if r != row {
                    grid[row][col] = '.';
                    grid[r][col] = 'O';
                }
            }
        }
    }

    grid.iter().enumerate().map(|(i,row)| {
        let load_multiplier = num_rows - i;
        row.iter().filter(|c| c==&&'O').count() * load_multiplier
    })
    .sum()
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
