use itertools::{repeat_n, join, Itertools};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7017);

    // let result2 = part2(input);
    // println!("Part 2: {result2}");
    // assert_eq!(result2, 7017);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (pattern, groups) = line.split_once(' ').unwrap();
            let groups: Vec<u32> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            let num_questions = pattern.chars().filter(|c| c == &'?').count() as u32;
            let num_hashes = groups.iter().sum::<u32>() - pattern.chars().filter(|c| c==&'#').count() as u32;
            let num_dots = num_questions - num_hashes;

            count_valid_arrangements(pattern, num_hashes, num_dots, &groups)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (pattern, groups) = line.split_once(' ').unwrap();
            let groups: Vec<u32> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            // Pattern is repeated 5 times, separated by '?'
            let pattern = join(repeat_n(pattern, 5), "?");
            // Groups is repeated 5 times
            let groups = repeat_n(groups, 5).flatten().collect_vec();

            let num_questions = pattern.chars().filter(|c| c == &'?').count() as u32;
            let num_hashes = groups.iter().sum::<u32>() - pattern.chars().filter(|c| c==&'#').count() as u32;
            let num_dots = num_questions - num_hashes;

            count_valid_arrangements(&pattern, num_hashes, num_dots, &groups)
        })
        .sum()
}

fn count_valid_arrangements(pattern: &str, num_hashes: u32, num_dots: u32, groups: &[u32]) -> u32 {
    // Try replacing the first question mark with a hash or dot, and compute
    // the number of valid arrangements for each.

    // If there's nothing left to replace, it must match the given groups
    if num_hashes == 0 && num_dots == 0 {
        let pattern_groups = pattern.chars().group_by(|c| c==&'#').into_iter().filter_map(|(key, group)| key.then_some(group.count() as u32)).collect_vec();
        if pattern_groups == groups {
            return 1;
        } else {
            return 0;
        }
    }

    let mut result = 0;

    if num_hashes > 0 {
        let next_pattern = pattern.replacen('?', "#", 1);
        result += count_valid_arrangements(&next_pattern, num_hashes-1, num_dots, groups);
    }

    if num_dots > 0 {
        let next_pattern = pattern.replacen('?', ".", 1);
        result += count_valid_arrangements(&next_pattern, num_hashes, num_dots-1, groups);
    }

    result
}

#[test]
fn test_arrangements_1() {
    assert_eq!(count_valid_arrangements("???.###", 2, 1, &[1,1,3]), 1);
}

#[test]
fn test_arrangements_2() {
    assert_eq!(count_valid_arrangements(".??..??...?##.", 3, 2, &[1,1,3]), 4);
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
