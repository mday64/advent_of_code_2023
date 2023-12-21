use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input, 64);
    println!("Part 1: {result1}");
    assert_eq!(result1, 3687);
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

#[cfg(test)]
static EXAMPLE1: &str = "
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
