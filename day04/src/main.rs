use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 26426);
}

fn part1(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        // Assuming that there are no duplicates on either side of the
        // vertical bar, the number of matches is the number of numbers
        // on the line (excluding the "Card n: " prefix), minus the number
        // of unique numbers.  Then we turn that into a score.
        let (_prefix, rest) = line.split_once(": ").expect("no colon?");
        let numbers: Vec<u32> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let total_count = numbers.len();
        let unique_numbers: HashSet<u32> = numbers.into_iter().collect();
        let unique_count = unique_numbers.len();
        if unique_count < total_count {
            // Compute 2 ** (total_count - unique_count - 1)
            1u32.rotate_left((total_count - unique_count - 1) as u32)
        } else {
            0
        }
    })
    .sum()
}

#[test]
fn example1() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(part1(input), 13);
}
