fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> u32 {
    0
}

#[test]
fn example1() {
    let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(part1(input), 4361);
}
