use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 13301);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 13301);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    assert_eq!(lines.next().unwrap(), "");
    let nodes: HashMap<&str, (&str, &str)> = lines.map(|line| {
        let (key, values) = line.split_once(" = ").unwrap();
        let values = values.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        (key, (left, right))
    }).collect();

    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let children = nodes.get(current).unwrap();
        current = match directions.next().unwrap() {
            'L' => children.0,
            'R' => children.1,
            _ => panic!("invalid direction")
        };
        steps += 1;
    }
    steps
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    assert_eq!(lines.next().unwrap(), "");
    let nodes: HashMap<&str, (&str, &str)> = lines.map(|line| {
        let (key, values) = line.split_once(" = ").unwrap();
        let values = values.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        (key, (left, right))
    }).collect();

    let mut steps = 0;
    let mut current: Vec<&str> = nodes.keys().filter(|key| key.ends_with('A')).cloned().collect();
    while !current.iter().all(|key| key.ends_with('Z')) {
        let direction = directions.next().unwrap();
        // println!("{:?}", current);
        for c in current.iter_mut() {
            let children = nodes.get(c).unwrap();
            *c = match direction {
                'L' => children.0,
                'R' => children.1,
                _ => panic!("invalid direction")
            };
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
static EXAMPLE_INPUT_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
static EXAMPLE_INPUT_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
static EXAMPLE_INPUT_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

#[test]
fn test_part1_ex1() {
    assert_eq!(part1(EXAMPLE_INPUT_1), 2);
}

#[test]
fn test_part1_ex2() {
    assert_eq!(part1(EXAMPLE_INPUT_2), 6);
}

#[test]
fn test_part2_ex3() {
    assert_eq!(part2(EXAMPLE_INPUT_3), 6);
}
