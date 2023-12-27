use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 2130);
}

//
// The input appears to be constructed such that there are relatively
// few places where you can make a decision about which way to proceed.
// The rest of the dots are just paths connecting those decision points.
// We could preprocess the input into a set of nodes and directed and
// weighted edges, and then just use a DFS or BFS to visit all valid
// combinations of decisions, keeping track of the longest route that
// gets to the exit.
//
fn part1(input: &str) -> u32 {
    let puzzle = parse_input(input);
    // for (src, v) in puzzle.neighbors.iter() {
    //     for (dest, dist) in v {
    //         println!("{src:?} -> {dest:?} in {dist} steps");
    //     }
    // }

    longest_path(puzzle.start, &puzzle)
}

fn longest_path(src: Point, puzzle: &Puzzle) -> u32 {
    if src == puzzle.end {
        return 0;
    }

    puzzle.neighbors
        .get(&src)
        .unwrap()
        .iter()
        .map(|&(neighbor, dist)| dist + longest_path(neighbor, puzzle))
        .max()
        .unwrap()
}

struct Puzzle {
    start: Point,
    end: Point,
    neighbors: HashMap<Point, Vec<(Point, u32)>>,
}

fn parse_input(input: &str) -> Puzzle {
    let grid: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut neighbors: HashMap<Point, Vec<(Point, u32)>> = HashMap::new();
    let last_row = grid.len() - 1;
    let mut start=(0,0);
    let mut end=(0,0);

    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if ch == &'.' {
                if row == 0 {
                    start = (row, col);
                } else if row == last_row {
                    end = (row, col);
                } else {
                    // See if this location has more than 2 non-wall neighbors
                    let num_neighbors = [(row-1, col), (row+1,col), (row,col-1), (row,col+1)]
                        .into_iter()
                        .filter(|&(r,c)| grid[r][c] != '#')
                        .count();
                    if num_neighbors > 2 {
                        // println!("({row},{col})");
                        neighbors.insert((row, col), vec![]);
                    }
                }
            }
        }
    }
    // println!("start=({},{})", start.0, start.1);
    // println!("end=({},{})", end.0, end.1);
    neighbors.insert(start, vec![]);
    neighbors.insert(end, vec![]);

    // Now find the directed connections between the branching points
    let nodes = neighbors.keys().copied().collect::<HashSet<_>>();
    for &node in &nodes {
        use Direction::*;

        // The exit doesn't have any outbound edges
        if node == end {
            continue;
        }

        // The order of neighbors below is important; the first one will work
        // for the starting point.  We'll take advantage of that fact to
        // break out of the loop before trying to access grid[-1].
        for direction in [Down, Left, Right, Up] {
            // Follow the path to the next node, if any; find its length.
            // If we found a path to another node, insert it now.
            if let Some((dest, distance)) = path_in_direction(node, direction, &nodes, &grid) {
                (*neighbors.get_mut(&node).unwrap()).push((dest, distance));
            }
            
            // The starting point only has a path going down.  Don't bother
            // trying other directions, since up would result in an out of
            // bounds access to grid[][].            
            if node == start { break }
        }
    }

    Puzzle { start, end, neighbors }
}

fn path_in_direction(start: Point, direction: Direction, nodes: &HashSet<Point>, grid: &[Vec<char>]) -> Option<(Point, u32)> {
    use Direction::*;

    // println!("path_in_direction: start={start:?} direction={direction:?}");

    let mut dest = start;
    let mut distance = 0;
    let mut possible_directions: &[Direction] = &[direction];

    loop {
        let dir;

        // See if we can move in one of the possible directions.  If not,
        // immediately return None.
        (dir, dest) = possible_directions
            .iter()
            .map(|dir| (*dir, point_in_direction(dest, *dir)))
            .find(|&(dir, point)| {
                let (row, col) = point;
                let ch = grid[row][col];
                match (dir, ch) {
                    (_, '#') => false,
                    (_, '.') => true,
                    (Right, '>') | (Left, '<') | (Up, '^') | (Down, 'v') => {
                        // We need to be able to take another step in the same
                        // direction to "slide down the slope", and to prevent
                        // us from trying to backtrack.
                        // distance += 1;
                        // point = point_in_direction(point, dir);
                        true
                    }
                    _ => false,
                }
            })?;
        
        // println!("    dir={dir:?} dest={dest:?}");
        distance += 1;
        if nodes.contains(&dest) {
            // println!("-> Some(dest={dest:?}, distance={distance:?})");
            break Some((dest, distance));
        }

        possible_directions = match dir {
            Down => &[Down, Left, Right],
            Up => &[Up, Left, Right],
            Left => &[Down, Up, Left],
            Right => &[Down, Up, Right],
        }
    }
}

fn point_in_direction(start: Point, direction: Direction) -> Point {
    use Direction::*;

    let (row, col) = start;
    match direction {
        Up => (row-1, col),
        Down => (row+1, col),
        Left => (row, col-1),
        Right => (row, col+1),
    }
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[cfg(test)]
static EXAMPLE1: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 94);
}
