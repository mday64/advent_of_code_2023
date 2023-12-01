fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1(input);
    println!("Part 1: {}", result1);
}

#[allow(clippy::filter_next)]
fn calibration_value(line: &str) -> u32 {
    line.chars().filter(|c| c.is_ascii_digit()).next().unwrap().to_digit(10).unwrap() * 10 +
    line.chars().filter(|c| c.is_ascii_digit()).last().unwrap().to_digit(10).unwrap()
}

fn part1(input: &str) -> u32 {
    input.lines().map(calibration_value).sum()
}

#[test]
fn example1() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
    assert_eq!(part1(input), 142);
}
