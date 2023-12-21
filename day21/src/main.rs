use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input, 64);
    println!("Part 1: {result1}");
    assert_eq!(result1, 3687);

    let result2 = part2(input, 26_501_365);
    println!("Part 2: {result2}");
    assert_eq!(result2, 3687);
}

fn part1(input: &str, steps: u32) -> usize {
    // Note: the input is constructed such that the starting point
    // is near the center, and the map is large enough that you can't
    // exit the map in the given number of steps.  Therefore, we don't
    // need to do any bounds checking, and we can use unsigned numbers
    // for coordinates.
    let mut start = None;
    let mut rocks = HashSet::new();
    let mut reachable = HashSet::new();

    // Parse the input, keeping track of the locations of rocks and
    // the starting point.
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Some((row, col));
            } else if ch == '#' {
                rocks.insert((row, col));
            }
        }
    }
    let start = start.expect("should find the starting point");

    reachable.insert(start);

    for _step in 0..steps {
        // Find any points reachable from the previous step
        reachable = reachable.into_iter().flat_map(|(row, col)| {
            [(row-1, col), (row, col-1), (row, col+1), (row+1, col)]
            .iter()
            .filter(|point| !rocks.contains(point))
            .copied()
            .collect::<Vec<_>>()
        }).collect();
    }

    reachable.len()
}

fn part2(input: &str, steps: u32) -> usize {
    // Note: the input grid repeats infinitely in all directions.
    // Note: the outer edges of the grid are never rocks.
    let mut start = None;
    let mut rocks = HashSet::new();
    let mut reachable = HashMap::new();

    // Parse the input, keeping track of the locations of rocks and
    // the starting point.
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Some((row as isize, col as isize));
            } else if ch == '#' {
                rocks.insert((row as isize, col as isize));
            }
        }
    }
    let start = start.expect("should find the starting point");
    let num_rows = input.lines().count() as isize;
    let num_cols = input.lines().next().unwrap().len() as isize;

    reachable.insert(start, 1);

    for _step in 0..steps {
        // Find any points reachable from the previous step
        let mut next_generation = HashMap::new();

        for ((row, col), count) in reachable {
            for (r,c) in [(row-1, col), (row, col-1), (row, col+1), (row+1, col)] {
                let r = (r + num_rows) % num_rows;
                let c = (c + num_cols) % num_cols;
                if !rocks.contains(&(r,c)) {
                    *next_generation.entry((r, c)).or_default() += count;
                }
            }
        }
        reachable = next_generation;
    }

    reachable.values().sum()
}

#[cfg(test)]
static EXAMPLE1: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1, 6), 16);
}

#[test]
fn test_part2_6() {
    assert_eq!(part2(EXAMPLE1, 6), 16);
}

#[test]
fn test_part2_10() {
    assert_eq!(part2(EXAMPLE1, 10), 50);
}

#[test]
fn test_part2_50() {
    assert_eq!(part2(EXAMPLE1, 50), 1594);
}

#[test]
fn test_part2_100() {
    assert_eq!(part2(EXAMPLE1, 100), 6536);
}

#[test]
fn test_part2_500() {
    assert_eq!(part2(EXAMPLE1, 500), 167004);
}

#[test]
fn test_part2_1000() {
    assert_eq!(part2(EXAMPLE1, 1000), 668697);
}

#[test]
fn test_part2_5000() {
    assert_eq!(part2(EXAMPLE1, 5000), 16733044);
}
