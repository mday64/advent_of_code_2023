use itertools::{repeat_n, join, Itertools};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7017);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    // assert_eq!(result2, 7017);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (pattern, groups) = line.split_once(' ').unwrap();
            let num_questions = pattern.chars().filter(|c| c == &'?').count();
            let groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();

            // Let's use brute force and ignorance!  Try all possible combinations
            // of '.' and '#' for each question mark.  Then see if it matches the
            // group lengths.
            repeat_n(".#".chars(), num_questions)
                .multi_cartesian_product()
                .map(|combination| {
                    let mut combination = combination.iter();
                    let possibility = pattern
                        .chars()
                        .map(|c| {
                            if c == '?' {
                                *combination.next().unwrap()
                            } else {
                                c
                            }
                        })
                        .collect_vec();
                    possibility
                        .iter()
                        .group_by(|ch| ch == &&'#')
                        .into_iter()
                        .filter_map(|(key, group)| key.then_some(group.count()))
                        .collect_vec()
                })
                .filter(|v| v == &groups)
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (pattern, groups) = line.split_once(' ').unwrap();
            let groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            // Pattern is repeated 5 times, separated by '?'
            let pattern = join(repeat_n(pattern, 5), "?");
            // Groups is repeated 5 times
            let groups = repeat_n(groups, 5).flatten().collect_vec();
            let num_questions = pattern.chars().filter(|c| c == &'?').count();

            // Let's use brute force and ignorance!  Try all possible combinations
            // of '.' and '#' for each question mark.  Then see if it matches the
            // group lengths.
            repeat_n(".#".chars(), num_questions)
                .multi_cartesian_product()
                .map(|combination| {
                    let mut combination = combination.iter();
                    let possibility = pattern
                        .chars()
                        .map(|c| {
                            if c == '?' {
                                *combination.next().unwrap()
                            } else {
                                c
                            }
                        })
                        .collect_vec();
                    possibility
                        .iter()
                        .group_by(|ch| ch == &&'#')
                        .into_iter()
                        .filter_map(|(key, group)| key.then_some(group.count()))
                        .collect_vec()
                })
                .filter(|v| v == &groups)
                .count()
        })
        .sum()
}

#[cfg(test)]
static EXAMPLE1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE1), 525152);
}

#[test]
fn test_part2a() {
    assert_eq!(part2("???.### 1,1,3"), 1);
}

#[test]
fn test_part2b() {
    assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
}

#[test]
fn test_part2c() {
    assert_eq!(part2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
}

#[test]
fn test_part2d() {
    assert_eq!(part2("????.#...#... 4,1,1"), 16);
}

#[test]
fn test_part2e() {
    assert_eq!(part2("????.######..#####. 1,6,5"), 2500);
}

#[test]
fn test_part2f() {
    assert_eq!(part2("?###???????? 3,2,1"), 506250);
}
