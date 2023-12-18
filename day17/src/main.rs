use std::collections::HashMap;
use pathfinding::directed::astar::astar;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 963);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 1178);
}

fn part1(input: &str) -> u32 {
    // Parse the input into a hashmap (which makes boundary detection easier)
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, value) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((row as i32, col as i32), value);
        }
    }
    let last_row = input.lines().count() as i32 - 1;
    let last_col = input.lines().next().unwrap().len() as i32 - 1;

    let neighbor_in_direction = |node: &Node, direction: Direction| -> Option<(Node, u32)> {
        let count = if node.direction == direction {
            node.count + 1
        } else {
            1
        };
        if count > 3 {
            return None;
        }

        let (row, col) = match direction {
            Direction::Up => (node.row - 1, node.col),
            Direction::Down => (node.row + 1, node.col),
            Direction::Left => (node.row, node.col - 1),
            Direction::Right => (node.row, node.col + 1),
        };

        map.get(&(row, col)).map(|&cost| (Node{row, col, direction, count}, cost))
    };
    let start = Node::new(0, 0, Direction::Right, 0);
    let successors = |node: &Node| -> Vec<(Node, u32)> {
        let mut result = vec![];
        if let Some(neighbor) = neighbor_in_direction(node, node.direction) {
            result.push(neighbor);
        }
        if let Some(neighbor) = neighbor_in_direction(node, node.direction.turn_left()) {
            result.push(neighbor);
        }
        if let Some(neighbor) = neighbor_in_direction(node, node.direction.turn_right()) {
            result.push(neighbor);
        }
        result
    };
    let heuristic = |node: &Node| -> u32 {
        ((last_row - node.row) + (last_col - node.col)) as u32
    };
    let success = |node: &Node| -> bool {
        (node.row == last_row) && (node.col == last_col)
    };

    let (_path, cost) = astar(&start, successors, heuristic, success).unwrap();
    // dbg!(_path);
    cost
}

fn part2(input: &str) -> u32 {
    // Parse the input into a hashmap (which makes boundary detection easier)
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, value) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((row as i32, col as i32), value);
        }
    }
    let last_row = input.lines().count() as i32 - 1;
    let last_col = input.lines().next().unwrap().len() as i32 - 1;

    let neighbor_in_direction = |node: &Node, direction: Direction| -> Option<(Node, u32)> {
        let count = if node.direction == direction {
            node.count + 1
        } else {
            1
        };
        if count > 10 {
            // Can't go straight more than 10 blocks
            return None;
        }
        if node.count > 0 && node.direction != direction && node.count < 4 {
            // Can't turn too quickly
            return None;
        }

        let (row, col) = match direction {
            Direction::Up => (node.row - 1, node.col),
            Direction::Down => (node.row + 1, node.col),
            Direction::Left => (node.row, node.col - 1),
            Direction::Right => (node.row, node.col + 1),
        };

        map.get(&(row, col)).map(|&cost| (Node{row, col, direction, count}, cost))
    };
    let start = Node::new(0, 0, Direction::Right, 0);
    let successors = |node: &Node| -> Vec<(Node, u32)> {
        let mut result = vec![];
        if let Some(neighbor) = neighbor_in_direction(node, node.direction) {
            result.push(neighbor);
        }
        if let Some(neighbor) = neighbor_in_direction(node, node.direction.turn_left()) {
            result.push(neighbor);
        }
        if let Some(neighbor) = neighbor_in_direction(node, node.direction.turn_right()) {
            result.push(neighbor);
        }
        result
    };
    let heuristic = |node: &Node| -> u32 {
        ((last_row - node.row) + (last_col - node.col)) as u32
    };
    let success = |node: &Node| -> bool {
        (node.row == last_row) && (node.col == last_col) && (node.count >= 4)
    };

    let (_path, cost) = astar(&start, successors, heuristic, success).unwrap();
    // dbg!(_path);
    cost
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    row: i32,
    col: i32,
    direction: Direction,
    count: u8,
}

impl Node {
    fn new(row: i32, col: i32, direction: Direction, count: u8) -> Self {
        Self { row, col, direction, count }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            }
    }
}

#[cfg(test)]
static EXAMPLE1: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 102);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE1), 94);
}
