fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 30487);
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    'outer: for pattern in input.split("\n\n") {
        let pattern: Vec<Vec<char>> = pattern.lines().map(|line| line.chars().collect()).collect();

        // See if there is a reflection about a row.  Here, `row` is the
        // number of rows before the reflection
        for row in 1..pattern.len() {
            // How many rows fit on both sides of `row`?
            let height = row.min(pattern.len() - row);
            if pattern[(row-height)..row].iter().eq(pattern[row..(row+height)].iter().rev()) {
                result += 100 * row;
                continue 'outer;
            }
        }

        // See if there is a reflection about a column.  Here, `col` is
        // the number of columns to the left of the reflection.
        let total_columns = pattern[0].len();
        for col in 1..total_columns {
            // How many columns fit on both sides of `col`?
            let width = col.min(total_columns - col);
            if pattern.iter().all(|row| row[(col-width)..col].iter().eq(row[col..(col+width)].iter().rev())) {
                result += col;
                continue 'outer;
            }
        }

        panic!("No reflection found!");
    }
    result
}

#[cfg(test)]
static EXAMPLE1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 405);
}
