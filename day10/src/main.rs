fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 6800);
}

fn part1(input: &str) -> u32 {
    // I think the trick here is to go around the loop until you get back
    // to the starting position.  The answer is half of that distance.

    // Parse the input into a Vec of Vecs (indexed as [row][col]).
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut start_row = 0;
    let mut start_col = 0;
    for (row, chars) in grid.iter().enumerate() {
        for (col, ch) in chars.iter().enumerate() {
            if ch == &'S' {
                start_row = row;
                start_col = col;
            }
        }
    }

    let mut distance = 0;
    let mut direction;
    let mut row = start_row;
    let mut col = start_col;

    // Look at the neighbors of 'S' to find a connected pipe.
    if row > 0 && "|7F".contains(grid[row-1][col]) {
        direction = Dir::Up;
    } else if col < num_cols-1 && "-J7".contains(grid[row][col+1]) {
        direction = Dir::Right;
    } else if row < num_rows-1 && "|LJ".contains(grid[row+1][col]) {
        direction = Dir::Down;
    } else if col > 0 && "-LF".contains(grid[row][col-1]) {
        direction = Dir::Left;
    } else {
        panic!("No connected pipe found!")
    }

    loop {
        // Move to the neighbor in direction `direction`
        match direction {
            Dir::Up => row -= 1,
            Dir::Down => row += 1,
            Dir::Left => col -= 1,
            Dir::Right => col += 1,
        };
        distance += 1;
        if row == start_row && col == start_col {
            break;
        }

        // Figure out the new direction to leave neighbor
        direction = match (direction, grid[row][col]) {
            (Dir::Up, '|') => Dir::Up,
            (Dir::Up, '7') => Dir::Left,
            (Dir::Up, 'F') => Dir::Right,
            (Dir::Down, '|') => Dir::Down,
            (Dir::Down, 'L') => Dir::Right,
            (Dir::Down, 'J') => Dir::Left,
            (Dir::Left, '-') => Dir::Left,
            (Dir::Left, 'L') => Dir::Up,
            (Dir::Left, 'F') => Dir::Down,
            (Dir::Right, '-') => Dir::Right,
            (Dir::Right, 'J') => Dir::Up,
            (Dir::Right, '7') => Dir::Down,
            _ => panic!("Invalid direction & char")
        };
    }
    distance / 2
}

enum Dir {
    Up,
    Right,
    Down,
    Left
}

#[cfg(test)]
static EXAMPLE1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

#[cfg(test)]
static EXAMPLE2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

#[test]
fn test_part1_ex1() {
    assert_eq!(part1(EXAMPLE1), 4);
}

#[test]
fn test_part1_ex2() {
    assert_eq!(part1(EXAMPLE2), 8);
}
