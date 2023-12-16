fn main() {
    let input = include_str!("../input.txt").trim_end();

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 517551);
}

fn part1(input: &str) -> u32 {
    input.split(',').map(|word| {
        word.chars().fold(0, |acc, elem| {
            ((acc + elem as u32) * 17) & 0xFF
        })
    }).sum()
}

#[cfg(test)]
static EXAMPLE1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 1320);
}
