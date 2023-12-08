fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    // assert_eq!(result1, 13);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .expect("at least one line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("valid number"))
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .expect("at least two lines")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("valid number"))
        .collect();

    let mut result = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        // Let's just brute force.  Try holding the button for 1 ms up to time-1,
        // and compute the resulting distance, comparing to the best distance.
        result *= (1..time)
            .map(|charge| charge * (time - charge))
            .filter(|dist| dist > &distance)
            .count() as u32;
    }
    result
}

#[cfg(test)]
static EXAMPLE_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 288);
}
