use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 13301);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 7309459565207);
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

//
// The answer for part 2 is sufficiently large that it is impractical to
// try to simulate the set of steps from each starting state in parallel.
//
// Let's call the answer N.  Since the total number of states is far
// smaller than N, each starting state will produce a cycle.  The answer,
// N, will be congruent to the number of steps until the cycle starts,
// modulo the number of steps in the cycle.  (Here, a cycle refers to
// a state that is a valid ending state; i.e., it ends with "Z".)
//
fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    assert_eq!(lines.next().unwrap(), "");
    let nodes: HashMap<&str, (&str, &str)> = lines.map(|line| {
        let (key, values) = line.split_once(" = ").unwrap();
        let values = values.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        (key, (left, right))
    }).collect();

    let starting_states: Vec<&str> = nodes.keys().filter(|key| key.ends_with('A')).cloned().collect();

    // Find the cycles for each of the starting states
    let cycles: Vec<_> = starting_states.into_iter().map(|initial| {
        let mut current = initial;
        let mut steps = 0;
        let mut seen: HashMap<&str, usize> = HashMap::new();
        while !seen.contains_key(current) {
            if current.ends_with('Z') {
                seen.insert(current, steps);
            }
            let children = nodes.get(current).unwrap();
            current = match directions.next().unwrap() {
                'L' => children.0,
                'R' => children.1,
                _ => panic!("invalid direction")
            };
            steps += 1;
        }
        let cycle_start = *seen.get(current).unwrap();
        let cycle_length = steps - cycle_start;
        (cycle_start, cycle_length)
    }).collect();

    // While debugging, I noticed that cycle_start == cycle_length for
    // each cycle.  That simplifies calculating the answer.  I'll leave
    // a more general solution for another time.
    assert!(cycles.iter().all(|(cycle_start, cycle_length)| cycle_start == cycle_length));

    cycles.iter().map(|(_start, length)| *length).reduce(num::integer::lcm).unwrap()
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
