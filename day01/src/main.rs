fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1(input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 54597);
    let result2 = part2(input);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 54504);
}

#[allow(clippy::filter_next)]
fn calibration_value(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().expect("must be at least one digit");
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

fn calibration_value2(line: &str) -> u32 {
    // A regex is *almost* the right solution, except that it will only find
    // non-overlapping matches, and can't directly find the rightmost match.
    // So it wouldn't work for "oneight" or "eightwo".
    //
    // Instead, we will explicitly search for all possible words at all
    // possible offsets.
    let mut digits = line.char_indices().filter_map(|(line_offset, c)| {
        let mut digit = c.to_digit(10);
        if digit.is_none() {
            digit = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
                .iter()
                .enumerate()
                .filter_map(|(word_index, word)| {
                    if line[line_offset..].starts_with(word) {
                        Some(word_index as u32 + 1)
                    } else {
                        None
                    }
                })
                .next()
        }
        digit
    });
    let first = digits.next().expect("must be at least one digit");
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

fn part1(input: &str) -> u32 {
    input.lines().map(calibration_value).sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(calibration_value2).sum()
}

#[test]
fn example1() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
    assert_eq!(part1(input), 142);
}

#[test]
fn example2() {
    let input = "\
    two1nine\n\
    eightwothree\n\
    abcone2threexyz\n\
    xtwone3four\n\
    4nineeightseven2\n\
    zoneight234\n\
    7pqrstsixteen\n";
    assert_eq!(part2(input), 281);
}
