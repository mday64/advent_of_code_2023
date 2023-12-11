use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 9403026);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 543018317006);
}

fn part1(input: &str) -> usize {
    let mut galaxies = Vec::<(usize, usize)>::new(); // (row, col)
    let mut occupied_rows = HashSet::<usize>::new();
    let mut occupied_cols = HashSet::<usize>::new();

    // Parse the input.  Find the location of all of the galaxies.
    // While we're parsing, also make note of which rows and columns
    // contain a galaxy.
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((row, col));
                occupied_rows.insert(row);
                occupied_cols.insert(col);
            }
        }
    }

    // Find the Manhattan distance between each pair of galaxies,
    // adjusted for empty rows and columns between them.  And sum
    // those all up.
    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (min_row, max_row) = minmax(pair[0].0, pair[1].0);
            let (min_col, max_col) = minmax(pair[0].1, pair[1].1);

            max_row - min_row + max_col - min_col
                + (min_row..max_row)
                    .map(|row| if occupied_rows.contains(&row) { 0 } else { 1 })
                    .sum::<usize>()
                + (min_col..max_col)
                    .map(|row| if occupied_cols.contains(&row) { 0 } else { 1 })
                    .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut galaxies = Vec::<(usize, usize)>::new(); // (row, col)
    let mut occupied_rows = HashSet::<usize>::new();
    let mut occupied_cols = HashSet::<usize>::new();

    // Parse the input.  Find the location of all of the galaxies.
    // While we're parsing, also make note of which rows and columns
    // contain a galaxy.
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((row, col));
                occupied_rows.insert(row);
                occupied_cols.insert(col);
            }
        }
    }

    // Find the Manhattan distance between each pair of galaxies,
    // adjusted for empty rows and columns between them.  And sum
    // those all up.
    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (min_row, max_row) = minmax(pair[0].0, pair[1].0);
            let (min_col, max_col) = minmax(pair[0].1, pair[1].1);

            max_row - min_row + max_col - min_col
                + (min_row..max_row)
                    .map(|row| if occupied_rows.contains(&row) { 0 } else { 1 })
                    .sum::<usize>() * 999999
                + (min_col..max_col)
                    .map(|row| if occupied_cols.contains(&row) { 0 } else { 1 })
                    .sum::<usize>() * 999999
        })
        .sum()
}

fn minmax(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
static EXAMPLE1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 374);
}
