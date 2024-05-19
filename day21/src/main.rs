use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input, 64);
    println!("Part 1: {result1}");
    assert_eq!(result1, 3687);

    let result2 = part2(input, 26_501_365);
    println!("Part 2: {result2}");
    // 610315846454025 is too low
    // 610316615797122 is too low
    // assert_eq!(result2, 610316615797122);
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

//
// The input repeats infinitely in all four directions.  We have to
// calculate how many of the open (non-rock) locations can be reached
// in the given number of steps.  For ease of discussion, I'm going
// to call each copy of the input a "section."
//
// Because steps are only up/down/left/right, and no diagonals, any
// given location can only be an odd number of steps or an even number
// of steps, but not both.  In fact, this even/odd-ness is the same
// as the Manhattan distance to that location.  That means the problem
// becomes finding the minimum number of steps at which a location
// becomes reachable.  Any location reachable with the input number
// of steps or less, and the same even/odd-ness, will be reachable
// in the last step (at worst, you just keep stepping back and forth
// to an adjacent open space).
//
// In our input, there is a direct, unobstructed path from the starting
// location to each edge, traveling in just one direction (all up, all down,
// etc.).  The edges are also unobstructed.
//
// I believe that the rocks are sparse enough that no interior location is
// a further path distance than that section's furthest corner.  Therefore,
// if the furthest corner has a Manhattan distance less than or equal to
// the input number of steps, then all empty (non-rock) locations with the
// correct odd/even-ness are reachable.  We can pre-compute these numbers
// (one for odd, one for even) in advance.
//
// For sections where the nearest edge/corner is within the given number
// of steps, but the furthest corner is not, then we have to explicitly
// count the number of steps from the closest location.
//
// In my input, the number of rows and columns are odd.  This means that
// the odd/even-ness of corresponding locations alternates every time
// you step through a section.  I think all sections on a diagonal will
// have the same odd/even-ness.
//
// Recall that for Manhattan distance, equal distance looks like a diamond
// (a square rotated by 45 degrees).  So all sections completely inside
// the given number of steps will form a filled-in diamond.  It's not quite
// as simple as caluclating the number of sections times the number of
// reachable locations because of the odd/even-ness.  So there will be two
// numbers of reachable locations, and we have to figure out how many sections
// of each to add.
//
fn part2(input: &str, steps: usize) -> usize {
    // Note: the input grid repeats infinitely in all directions.
    // Note: the outer edges of the grid are never rocks.
    let mut start = None;
    let mut rocks = HashSet::new();

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
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();

    // The starting position should be exactly in the middle of the input.
    assert!(num_rows & 1 == 1);
    assert!(num_cols & 1 == 1);
    assert!(num_rows == start.0 * 2 + 1);
    assert!(num_cols == start.1 * 2 + 1);

    // Simplifying assumption: the input is square, so distances are
    // the same in all directions.
    assert_eq!(num_rows, num_cols);
    assert_eq!(start.0, start.1);
    let start = start.0;
    let dimension = num_rows;

    // There should be no rocks from the starting point to any edge.
    // There should be no rocks on the outer edges.
    for r in 0..num_rows {
        assert!(!rocks.contains(&(r, start)));
        assert!(!rocks.contains(&(r, 0)));
        assert!(!rocks.contains(&(r, num_cols - 1)));
    }
    for c in 0..num_cols {
        assert!(!rocks.contains(&(start, c)));
        assert!(!rocks.contains(&(0, c)));
        assert!(!rocks.contains(&(num_rows - 1, c)));
    }

    // Precompute the number of steps from any corner or edge middle to
    // any non-rock.  The furthest corner should have the maximum distance.
    // The key of the outer map is (r, c), where r and c are the signum of
    // the number of sections relative to the starting section (r = rows,
    // c = columns).  For example, (0, -1) would be the same row as the
    // starting section, and to the left.  The origin, the nearest point
    // to the starting point, would be the middle of the right edge.
    // The value is a vector of the number of steps to every non-rock.
    let mut reachable_from = HashMap::<(isize, isize), Vec<usize>>::new();
    reachable_from.insert((0, 0), shortest_paths(start, start, dimension, &rocks));
    reachable_from.insert((0, 1), shortest_paths(start, 0, dimension, &rocks));
    reachable_from.insert((0, -1), shortest_paths(start, dimension-1, dimension, &rocks));
    reachable_from.insert((1, 0), shortest_paths(0, start, dimension, &rocks));
    reachable_from.insert((-1, 0), shortest_paths(dimension-1, start, dimension, &rocks));
    reachable_from.insert((-1, -1), shortest_paths(dimension-1, dimension-1, dimension, &rocks));
    reachable_from.insert((-1, 1), shortest_paths(dimension-1, 0, dimension, &rocks));
    reachable_from.insert((1, -1), shortest_paths(0, dimension-1, dimension, &rocks));
    reachable_from.insert((1, 1), shortest_paths(0, 0, dimension, &rocks));

    // Calculate how many locations are reachable with an even or odd number
    // number of steps.
    // TODO: Since every corner and edge is reachable from the center,
    // the even and odd counts contain only two values.  The counts for
    // the center origin and corner origins are all the same.  The counts
    // for the edge origins are swapped odd <-> even.
    let mut even_counts = HashMap::<(isize, isize), usize>::new();
    let mut odd_counts = HashMap::<(isize, isize), usize>::new();
    for (&(dr, dc), v) in reachable_from.iter() {
        let mut num_even = 0;
        let mut num_odd = 0;
        for n in v {
            if n & 1 == 1 {
                num_odd += 1;
            } else {
                num_even += 1;
            }
        }
        even_counts.insert((dr, dc), num_even);
        odd_counts.insert((dr, dc), num_odd);
    }
    assert_eq!(even_counts[&(0, 1)], even_counts[&(0, -1)]);
    assert_eq!(even_counts[&(0, 1)], even_counts[&(1, 0)]);
    assert_eq!(even_counts[&(0, 1)], even_counts[&(-1, 0)]);
    assert_eq!(even_counts[&(1, 1)], even_counts[&(-1, -1)]);
    assert_eq!(even_counts[&(1, 1)], even_counts[&(1, -1)]);
    assert_eq!(even_counts[&(1, 1)], even_counts[&(-1, 1)]);
    assert_eq!(odd_counts[&(0, 1)], odd_counts[&(0, -1)]);
    assert_eq!(odd_counts[&(0, 1)], odd_counts[&(1, 0)]);
    assert_eq!(odd_counts[&(0, 1)], odd_counts[&(-1, 0)]);
    assert_eq!(odd_counts[&(1, 1)], odd_counts[&(-1, -1)]);
    assert_eq!(odd_counts[&(1, 1)], odd_counts[&(1, -1)]);
    assert_eq!(odd_counts[&(1, 1)], odd_counts[&(-1, 1)]);
    // The two below are true because `start` is odd.  There are `start` steps
    // from a corner to the middle of an edge.
    assert_eq!(even_counts[&(0,1)], odd_counts[&(1,1)]);
    assert_eq!(odd_counts[&(0,1)], even_counts[&(1,1)]);
    let both_counts = even_counts[&(0, 1)] + odd_counts[&(0, 1)];
    let counts = [even_counts, odd_counts];
    
    // Count the locations reachable in the starting section
    let mut parity = steps & 1;
    assert!(steps >= dimension);
    let mut result = counts[parity][&(0, 0)];
    println!("Starting section only: {}", result);
    assert_eq!(result, 7423);

    // Count the locations reachable from sections directly up/down/left/right
    // of the starting section.
    //
    // Figure out how many pairs of sections (even and odd offsets) are
    // completely within `steps`.  We can just multiply instead of iterating.
    // To get from start to any of the corners is start + start == dimension - 1.
    // For N pairs of sections, the far corners are 2 * N * dimension + dimension - 1.
    // (2*N+1) * dimension - 1 <= steps
    // (2*N+1) * dimension <= steps + 1
    // 2*N+1 <= (steps + 1) / dimension     // note integer rounding to zero!
    // 2*N <= (steps + 1) / dimension - 1
    // N <= ((steps + 1) / dimension - 1) / 2   // integer rounding to zero!
    let num_section_pairs = ((steps + 1) / dimension - 1) / 2;
    result += both_counts * num_section_pairs * 4;      // all 4 directions
    let mut origin_steps = start + 1 + 2 * dimension * num_section_pairs;
    while origin_steps <= steps {
        for origin in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            // TODO: If the furthest corners are within `steps`, then just
            // add even_counts or odd_counts, depending on parity
            for dist in &reachable_from[&(origin)] {
                let total_steps = origin_steps + dist;
                if total_steps <= steps && (total_steps ^ steps) & 1 == 0 {
                    result += 1;
                }
            }
        }
        origin_steps += dimension;
    }
    println!("Starting section plus cardinal sections: {}", result);
    assert_eq!(result, 6033799933);

    // Count the locations reachable from sections not in the same row or column
    // as the starting section.
    //
    // The closest such sections are diagonally adjacent.  Their origin (closest
    // location) is 2 * (start + 1) == dimension + 1 from the starting location.
    // Their furthest location is 2 * (start + 1) + 2 * (dimension - 1) ==
    // 2 * (start + dimension) == 3 * dimension - 1. There are 4 such sections,
    // 1 in each quadrant.  These have the same parity as the starting section.
    //
    // The next closest sections are exactly one section away.  The closest
    // location is 2 * dimension + 1.  The furthest is 4 * dimension - 1.
    // There are two such sections per quadrant (two rows and one column, or
    // one row and two columns from the starting section).
    //
    // To figure out the furthest diagonal completely within `steps`
    // (where N is the number of sections per quadrant):
    // (N + 2) * dimension - 1 <= steps
    // (N + 2) * dimension <= steps + 1
    // N + 2 <= (steps + 1) / dimension     // integer division rounds down
    // N <= (steps + 1) / dimension - 2
    //
    // The diagonals with 1, 3, 4, 5, ..., N sections per quadrant have the
    // same parity as the starting section.  The diagonals with 2, 4, 6, ..., N
    // sections per quadrant have the opposite parity.
    //
    // The sum of odd numbers from 1 to 2k-1, inclusive, is k^2.
    // The sum of even numbers from 2 to 2k, inclusive, is k(k+1).
    //
    // TODO: Instead of looping over full sections, use a closed form
    // expression (sum of all even/odd numbers).
    parity = steps & 1;
    let mut min_steps = 2 * dimension + 1;
    let mut max_steps = 3 * dimension - 1;
    let mut diagonal_length = 1;
    while max_steps <= steps {
        result += counts[parity][&(1, 1)] * diagonal_length * 4;
        diagonal_length += 1;
        min_steps += dimension;
        max_steps += dimension;
        parity = 1 - parity;
    }
    while min_steps <= steps {
        for origin in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            for dist in &reachable_from[&(origin)] {
                let total_steps = min_steps + dist;
                if total_steps <= steps && (total_steps ^ steps) & 1 == 0 {
                    result += diagonal_length;
                }
            }
        }
        
        diagonal_length += 1;
        min_steps += dimension;
    }
    println!("Including diagonal sections: {}", result);
    assert_eq!(result, 610316615797122);

    result
}

fn shortest_paths(row: usize, col: usize, dimension: usize, rocks: &HashSet<(usize, usize)>) -> Vec<usize> {
    let row = row as isize;
    let col = col as isize;
    let dimension = dimension as isize;
    let mut result = HashMap::<(isize, isize), usize>::new();
    let mut frontier = VecDeque::<(isize, isize)>::new();
    result.insert((row, col), 0);
    frontier.push_back((row, col));

    while let Some((r, c)) = frontier.pop_front() {
        let dist = *result.get(&(r, c)).unwrap();
        for (r, c) in [(r-1, c), (r+1, c), (r, c-1), (r, c+1)] {
            if r >= 0 && r < dimension && c >= 0 && c < dimension
            && !result.contains_key(&(r, c)) && !rocks.contains(&(r as usize, c as usize)) {
                result.insert((r, c), dist+1);
                frontier.push_back((r, c));
            }
        }
    }

    result.values().cloned().collect()
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
