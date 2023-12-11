fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 6800);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 483);
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

fn part2(input: &str) -> u32 {
    //
    // I think the solution here is to make a new grid with just the
    // loop marked.  Then pick a location outside the bounds of the
    // loop and find all reachable coordinates; mark all these.
    // What's left unmarked are the inside locations; count them.
    //
    // In order for the reachability part to "squeeze between pipes",
    // I'm going to "zoom in" by a factor of 2 so that the new grid
    // will include grid points in between pipes.  I also want to make
    // sure there is a border of at least 1 space around the loop.
    // It might be worth having an additional border that is pre-marked
    // so that I don't have to bother checking whether coordinates are
    // in bounds.
    //

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

    // Make the "zoomed in" grid where we mark the pipes, and locations
    // known to be exterior to the loop.  The dimensions are 2N+3, where
    // N is the dimension of the original grid.
    let mut marks = (0..(2*num_rows+3)).map(|_row| {
        vec![false; 2*num_cols+3]
    }).collect::<Vec<_>>();
    // Mark the outermost rows and columns
    for col in 0..(2*num_cols+3) {
        marks[0][col] = true;
        marks[2*num_rows+2][col] = true;
    }
    for row in 1..(2*num_rows+2) {
        marks[row][0] = true;
        marks[row][2*num_cols+2] = true;
    }

    // Follow the loop, like in part 1, marking off the pipe locations.
    // A pipe at [row][col] in the original grid will be located at
    // [2*row+2][2*col+2].  Every move needs to mark two squares in
    // the given direction.
    marks[2*start_row+2][2*start_col+2] = true;
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
            Dir::Up => {
                marks[2*row+1][2*col+2] = true;
                marks[2*row][2*col+2] = true;
                row -= 1;
            },
            Dir::Down => {
                marks[2*row+3][2*col+2] = true;
                marks[2*row+4][2*col+2] = true;
                row += 1;
            },
            Dir::Left => {
                marks[2*row+2][2*col+1] = true;
                marks[2*row+2][2*col] = true;
                col -= 1;
            },
            Dir::Right => {
                marks[2*row+2][2*col+3] = true;
                marks[2*row+2][2*col+4] = true;
                col += 1;
            },
        };
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

    // Now pick a location outside the loop's bounds (that MUST be
    // outside), and mark it and every location reachable from it.
    mark_outside(&mut marks);

    // Now count how many locations are NOT marked.  Note that we
    // only examine even-numbered coordinates, since those were
    // potential pipe grid locations (odd-numbered coordinates are
    // "between pipes").
    marks.iter().step_by(2).skip(1).map(|row| {
        row.iter().step_by(2).skip(1).map(|b| {
            match b {
                true => 0,
                false => 1,
            }
        }).sum::<u32>()
    }).sum()
}

fn mark_outside(marks: &mut [Vec<bool>]) {
    let mut stack = vec![(1,1)];
    marks[1][1] = true;

    while let Some((row, col)) = stack.pop() {
        for (r,c) in [(row-1,col), (row+1,col), (row,col-1), (row,col+1)] {
            if !marks[r][c] {
                marks[r][c] = true;
                stack.push((r,c));
            }
        }
    }
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

#[cfg(test)]
static EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

#[cfg(test)]
static EXAMPLE4: &str = "\
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
";

#[cfg(test)]
static EXAMPLE5: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

#[cfg(test)]
static EXAMPLE6: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[test]
fn test_part2_ex3() {
    assert_eq!(part2(EXAMPLE3), 4);
}

#[test]
fn test_part2_ex4() {
    assert_eq!(part2(EXAMPLE4), 4);
}

#[test]
fn test_part2_ex5() {
    assert_eq!(part2(EXAMPLE5), 8);
}

#[test]
fn test_part2_ex6() {
    assert_eq!(part2(EXAMPLE6), 10);
}
