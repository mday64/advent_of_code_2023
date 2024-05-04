use itertools::{repeat_n, join, Itertools};
use std::iter::zip;
use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7017);

    let result2 = part2(input);
    println!("Part 2: {result2}");
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
        .enumerate()
        .par_bridge()
        .map(|(line_number, line)| {
            let start_time = std::time::Instant::now();

            let (pattern, groups) = line.split_once(' ').unwrap();
            let groups: Vec<u32> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            // Pattern is repeated 5 times, separated by '?'
            let pattern = join(repeat_n(pattern, 5), "?");
            // Groups is repeated 5 times
            let groups = repeat_n(groups, 5).flatten().collect_vec();

            let num_questions = pattern.chars().filter(|c| c == &'?').count() as u32;
            let num_hashes = groups.iter().sum::<u32>() - pattern.chars().filter(|c| c==&'#').count() as u32;
            let num_dots = num_questions - num_hashes;

            let result = count_valid_arrangements(&pattern, num_hashes, num_dots, &groups);

            let duration = start_time.elapsed().as_secs_f64();
            println!("{duration:15.9}: {line_number:3} {line} -> {result}");

            result
        })
        .sum()
}

fn count_valid_arrangements(pattern: &str, num_hashes: u32, num_dots: u32, groups: &[u32]) -> u32 {
    // Try replacing the first question mark with a hash or dot, and compute
    // the number of valid arrangements for each.

    // If the current pattern's initial groups don't match the given groups,
    // then there can't be any arrangements, no matter how the substitutions
    // happen.
    if !zip(initial_groups(pattern), groups).all(|(a,b)| &a == b) {
        return 0;
    }

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

fn initial_groups(pattern: &str) -> Vec<u32> {
    let mut result = vec![];

    let mut hashes = 0;
    for c in pattern.chars() {
        match c {
            '.' => {
                if hashes > 0 {
                    result.push(hashes);
                    hashes = 0;
                }
            }
            '#' => {
                hashes += 1;
            }
            '?' => {
                // Don't try to count a partial group
                break;
            }
            _ => {
                panic!("invalid character in pattern");
            }
        }
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

#[test]
fn test_part2_line4() {
    assert_eq!(part2("??.???#???? 1,4,1"), 5595385);
}

#[test]
fn test_part2_line73() {
    assert_eq!(part2("????#?#??????#??? 1,3,1,1,4"), 32692514);
}

#[test]
fn test_part2_line121() {
    assert_eq!(part2(".??????????????#??? 1,7,5,1"), 705862);
}

#[test]
fn test_part2_line16() {
    // I don't know the actual answer here, but I do know
    // that it takes a very long time, and the answer must
    // be very large.
    assert_eq!(part2("..?.????#?????????? 1,1,1,1,1,4"), 0);
}
