fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 2075724761);
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|seq| next_in_sequence(&seq))
        .sum()
}

fn next_in_sequence(seq: &[i32]) -> i32 {
    let diffs = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let next_diff = if diffs.iter().all(|&v| v == 0) {
        0
    } else {
        next_in_sequence(&diffs)
    };
    seq.last().unwrap() + next_diff
}

#[cfg(test)]
static EXAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 114);
}
