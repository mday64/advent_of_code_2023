use itertools::{repeat_n, join, Itertools};
use std::iter::zip;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 7017);

    let start_time = std::time::Instant::now();
    let result2 = part2(input);
    let duration = start_time.elapsed().as_secs_f64();
    println!("Part 2: {result2} in {duration:.9} seconds");
    assert_eq!(result2, 527570479489);
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

fn part2(input: &str) -> u64 {
    input
        .lines()
        .enumerate()
        //.par_bridge()
        .map(|(_line_number, line)| {
            let (pattern, groups) = line.split_once(' ').unwrap();
            let groups: Vec<u32> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            // Pattern is repeated 5 times, separated by '?'
            let pattern = join(repeat_n(pattern, 5), "?");
            // Groups is repeated 5 times
            let groups = repeat_n(groups, 5).flatten().collect_vec();

            let num_questions = pattern.chars().filter(|c| c == &'?').count() as u32;
            let num_hashes = groups.iter().sum::<u32>() - pattern.chars().filter(|c| c==&'#').count() as u32;
            let num_dots = num_questions - num_hashes;

            //let result = count_valid_arrangements(&pattern, num_hashes, num_dots, &groups);
            let mut cache = HashMap::new();
            let result = count_matches(pattern.as_bytes(), &groups, 0, 0, num_dots, &mut cache);

            result
        })
        .sum()
}

fn count_matches(
    pattern: &[u8], groups: &[u32],
    pattern_offset: usize, group_offset: usize,
    num_dots: u32,
    cache: &mut HashMap<(usize, usize), u64>
) -> u64 {
    if let Some(result) = cache.get(&(pattern_offset, group_offset)) {
        return *result
    }
    
    let orig_pattern_offset = pattern_offset;
    let mut pattern_offset = pattern_offset;
    let mut num_dots = num_dots;
    let mut result = 0;

    assert!(pattern_offset <= pattern.len());
    assert!(group_offset <= groups.len());

    // If we have used up all the groups, then the remainder of the pattern must
    // not contain any hashes.
    if group_offset == groups.len() {
        for c in &pattern[pattern_offset..] {
            if *c as char == '#' {
                // Impossible match
                cache.insert((orig_pattern_offset, group_offset), 0);
                return 0;
            }
        }
        // It was all dots or question marks, or empty
        cache.insert((orig_pattern_offset, group_offset), 1);
        return 1;
    }

    // Skip over leading dots in the remaining pattern
    while pattern_offset < pattern.len() && pattern[pattern_offset] as char  == '.' {
        pattern_offset += 1;
    }
    assert!(pattern_offset < pattern.len());

    if num_dots > 0 && pattern[pattern_offset] as char == '?' {
        // Try using the question mark as a dot
        result += count_matches(pattern, groups, pattern_offset+1, group_offset, num_dots-1, cache);
    }

    // Try to match the next group at the start of the remaining pattern.
    // The next group_len bytes of the pattern must not contain a dot.
    // If there is more pattern after that, it must not be a hash.
    let group_len = groups[group_offset] as usize;
    assert!(pattern.len() - pattern_offset >= group_len);
    if pattern[pattern_offset..pattern_offset+group_len].iter().all(|c| *c as char != '.') {
        let match_offset = pattern_offset;
        pattern_offset += group_len;
        if pattern_offset == pattern.len() || pattern[pattern_offset] as char == '.' || (num_dots > 0 && pattern[pattern_offset] as char != '#') {
            if pattern_offset < pattern.len() {
                if pattern[pattern_offset] as char == '?' {
                    num_dots -= 1;
                }
                pattern_offset += 1;
            }
            result += count_matches(pattern, groups, pattern_offset, group_offset+1, num_dots, cache);
        }
    }

    cache.insert((orig_pattern_offset, group_offset), result);
    result
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
fn test_part2_line3() {
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
fn part2_very_fast() {
    assert_eq!(part2("##.#????.# 2,4,1"), 1);
    assert_eq!(part2(".#?#??.#?? 3,2"), 1);
    assert_eq!(part2(".??#.?#?#... 3,4"), 1);
    assert_eq!(part2("?#??#.##??? 4,4"), 1);
    assert_eq!(part2("#.##???...?#.? 1,5,1,1"), 1);
    assert_eq!(part2("##???#.?..#?#? 6,1,3"), 1);
    assert_eq!(part2("#?#?#??.?#?# 3,3,3"), 1);
    assert_eq!(part2("#.#??#??#?????? 1,12"), 1);
    assert_eq!(part2("#.##????## 1,3,2"), 1);
    assert_eq!(part2("?#.??#?#?? 2,2,2"), 1);
    assert_eq!(part2("#?#?#?##..?.#?#?#?.? 3,4,5,1"), 1);
    assert_eq!(part2("..??..##.??? 2,2,3"), 16);
    assert_eq!(part2("..#?#??#??#?? 7,2"), 1);
    assert_eq!(part2(".?#??#???#?#???# 3,2,6,1"), 1);
    assert_eq!(part2("?..#??#??##. 2,1,2"), 1);
    assert_eq!(part2("??##..??#?#??# 4,1,1,4"), 1);
    assert_eq!(part2("?.#??.##??#????#?? 1,11"), 1);
    assert_eq!(part2("....#?##????.??#?? 4,1"), 1);
    assert_eq!(part2("?#?.?#?#?. 1,4"), 32);
    assert_eq!(part2("..?#.??#?#?????#?. 1,9"), 1);
    assert_eq!(part2("#.???##?.?.?# 1,5,2"), 32);
    assert_eq!(part2("??##???.?#??##. 7,5"), 16);
    assert_eq!(part2("???..?#???#???????? 3,14"), 1);
    assert_eq!(part2("##?.#....?.? 3,1,1"), 162);
    assert_eq!(part2("#?#?.?????. 4,1,3"), 16);
    assert_eq!(part2("??#???#?##??# 2,1,4,1"), 1);
    assert_eq!(part2("#?##??.##??????? 5,8"), 16);
    assert_eq!(part2("?#.#???.????? 1,3,5"), 16);
    assert_eq!(part2(".?.?#?#???#? 1,8"), 162);
    assert_eq!(part2("??????#.#???#?#?. 6,2,4"), 32);
}

#[test]
fn part2_1ms() {
    assert_eq!(part2(".??.???... 2,2"), 32);
    assert_eq!(part2(".?#??#???? 1,3"), 32);
    assert_eq!(part2("??.#?.??#?####? 2,7"), 32);
    assert_eq!(part2("#...?..?????..#. 1,1,3,1,1"), 1);
    assert_eq!(part2("??????#...????#?.? 6,5"), 32);
    assert_eq!(part2("#?#?#?.?#?????# 5,3,1"), 32);
    assert_eq!(part2("?#?.?????##.# 2,6,1"), 32);
    assert_eq!(part2("???#???#.????# 6,3,1"), 16);
    assert_eq!(part2("##??????#?? 6,3"), 533);
    assert_eq!(part2(".#???..#?. 1,1,2"), 252);
    assert_eq!(part2(".?##????#???#??#?? 9,4"), 32);
    assert_eq!(part2(".???.?.??? 3,2"), 162);
    assert_eq!(part2("#.?.????.#????? 1,4,1,4"), 16);
    assert_eq!(part2("?#???#?#??? 1,5"), 243);
    assert_eq!(part2("??#???#?.? 3,1"), 243);
    assert_eq!(part2(".#???#??#?#??????.?? 15,1"), 162);
    assert_eq!(part2("?????.?#?.####? 2,2,2,5"), 32);
    assert_eq!(part2("?.###??.??#?????? 4,8"), 162);
    assert_eq!(part2("?.?#??#?##??? 1,4,3"), 81);
    assert_eq!(part2("##????.?.###.? 2,1,3"), 1024);
    assert_eq!(part2("??.?#????#??#??.#?? 1,8,2"), 32);
    assert_eq!(part2("??##????#?#??????# 9,5,1"), 1);
    assert_eq!(part2("???#??..#????.????.# 1,2,1,5,3,1"), 32);
    assert_eq!(part2("?#???#???##??.??#? 12,1"), 162);
}

#[test]
fn part2_10ms() {
    assert_eq!(part2(".??.???... 2,2"), 32);
    assert_eq!(part2(".?#??#???? 1,3"), 32);
    assert_eq!(part2("??.#?.??#?####? 2,7"), 32);
    assert_eq!(part2("#...?..?????..#. 1,1,3,1,1"), 1);
    assert_eq!(part2("??????#...????#?.? 6,5"), 32);
    assert_eq!(part2("#?#?#?.?#?????# 5,3,1"), 32);
    assert_eq!(part2("?#?.?????##.# 2,6,1"), 32);
    assert_eq!(part2("???#???#.????# 6,3,1"), 16);
    assert_eq!(part2("##??????#?? 6,3"), 533);
    assert_eq!(part2(".#???..#?. 1,1,2"), 252);
    assert_eq!(part2(".?##????#???#??#?? 9,4"), 32);
    assert_eq!(part2(".???.?.??? 3,2"), 162);
    assert_eq!(part2("#.?.????.#????? 1,4,1,4"), 16);
    assert_eq!(part2("?#???#?#??? 1,5"), 243);
    assert_eq!(part2("??#???#?.? 3,1"), 243);
    assert_eq!(part2(".#???#??#?#??????.?? 15,1"), 162);
    assert_eq!(part2("?????.?#?.####? 2,2,2,5"), 32);
    assert_eq!(part2("?.###??.??#?????? 4,8"), 162);
    assert_eq!(part2("?.?#??#?##??? 1,4,3"), 81);
    assert_eq!(part2("##????.?.###.? 2,1,3"), 1024);
    assert_eq!(part2("??.?#????#??#??.#?? 1,8,2"), 32);
    assert_eq!(part2("??##????#?#??????# 9,5,1"), 1);
    assert_eq!(part2("???#??..#????.????.# 1,2,1,5,3,1"), 32);
    assert_eq!(part2("?#???#???##??.??#? 12,1"), 162);
}

#[test]
fn part2_50ms() {
    assert_eq!(part2(".????????#?. 6,2"), 243);
    assert_eq!(part2("?.???.?#?#. 1,3"), 2500);
    assert_eq!(part2("??????????####.#?? 3,8,3"), 243);
    assert_eq!(part2("????#???#????# 11,1"), 32);
    assert_eq!(part2(".??##??..?###?#??? 3,8"), 1024);
    assert_eq!(part2("#?#.???#???#?#.????# 1,1,10,1,1"), 243);
    assert_eq!(part2("??..???.???##??#.??# 1,3,4,2,1"), 162);
    assert_eq!(part2(".???##????#.#?.??#? 1,5,1,1,1,2"), 243);
    assert_eq!(part2("?#???#??##???? 1,7"), 243);
    assert_eq!(part2("#??.???##? 1,1,3"), 3125);
    assert_eq!(part2("???#.????.?#?#? 4,1,3"), 1024);
    assert_eq!(part2("??.#?..#?????? 2,2,4"), 1250);
    assert_eq!(part2(".?.?#????????##? 6,5"), 1875);
    assert_eq!(part2(".????.???#?.# 4,1,1,1"), 32);
    assert_eq!(part2("??#??###.????#??.??? 1,6,2,3,3"), 243);
    assert_eq!(part2("??.?#?.?.#??#? 2,1,5"), 4339);
    assert_eq!(part2(".??#??.??# 3,2"), 3888);
    assert_eq!(part2(".#???#??#???#???? 1,1,1,8,1"), 81);
    assert_eq!(part2(".?##????????###?. 2,1,2,5"), 1024);
    assert_eq!(part2("#???.#?.??#??????#?# 3,2,5,4"), 768);
    assert_eq!(part2("???????#????.?#??# 9,4"), 1024);
}

#[test]
fn part2_100ms() {
    assert_eq!(part2("???#?????.#?# 6,1,1,1"), 243);
    assert_eq!(part2(".#??.??.????###????? 1,1,2,8,3"), 81);
    assert_eq!(part2("?#.???#?.?? 1,3,1"), 10408);
    assert_eq!(part2("?#.???.?#? 2,1,2"), 7776);
    assert_eq!(part2("?#????#????.# 7,1,1"), 3988);
    assert_eq!(part2("#?????.?.?#?.??..? 6,2,2"), 2592);
    assert_eq!(part2("???#??#???.??#???#? 1,8,1,1,3"), 512);
    assert_eq!(part2("?.#?##??#.?#????? 1,5,1,2,3"), 768);
    assert_eq!(part2("?#?##????#??.#?# 5,4,1,1"), 3125);
    assert_eq!(part2("#??.??.???#?#? 3,1,6"), 5184);
    assert_eq!(part2("?#..#?.?#?#?????? 1,1,2,5"), 4025);
    assert_eq!(part2(".##?#???#?????.? 5,1,2"), 3443);
}

#[test]
fn part2_200ms() {
    assert_eq!(part2("???.?.?#??##????? 2,10"), 2500);
    assert_eq!(part2("??#?????#??#? 5,5"), 5741);
    assert_eq!(part2("???????#?????#..?? 5,2"), 1024);
    assert_eq!(part2("??.#????#?#?? 2,1,5"), 7588);
    assert_eq!(part2("?????#????.?????##?? 10,5"), 3888);
    assert_eq!(part2("???.#??????#??#?.. 2,3,6"), 5184);
    assert_eq!(part2("??#??????.?##? 6,3"), 14406);
    assert_eq!(part2("?###?#?????.????#?#? 10,4"), 5184);
    assert_eq!(part2("..?.??##?#??????##?? 1,14"), 6973);
    assert_eq!(part2("??#???.#???#?#??? 1,1,1,9"), 512);
    assert_eq!(part2("?#???.#????.?? 4,2,1,2"), 5184);
    assert_eq!(part2("?????.#?????? 1,7"), 11525);
}

#[test]
fn part2_500ms() {
    assert_eq!(part2("??#?????##??#????. 1,9"), 3125);
    assert_eq!(part2("?????#.#.??.#.???? 2,3,1,1,1,3"), 5184);
    assert_eq!(part2("???##?#####?????# 11,2"), 9604);
    assert_eq!(part2(".?#???#????#? 1,1,1,3"), 5998);
    assert_eq!(part2(".??#??#???###?????? 1,12,1,1"), 81);
    assert_eq!(part2("???#?.?#???#??? 4,3,2"), 7776);
    assert_eq!(part2(".#?????#????#? 1,5,2"), 16807);
    assert_eq!(part2("..#?????????? 3,3"), 17550);
    assert_eq!(part2("??????##??#? 1,1,7"), 8562);
    assert_eq!(part2("???.??##??? 1,6"), 52774);
    assert_eq!(part2(".#.???##??#??? 1,6,1,1"), 2420);
    assert_eq!(part2("?#????#???#?.. 1,1,1,3"), 10584);
}

#[test]
fn part2_1s() {
    assert_eq!(part2("?.#??????.#????#?? 1,1,1,1,1,7"), 16);
    assert_eq!(part2("#??.?##????#????? 3,8,2"), 32805);
    assert_eq!(part2("???.??#??#?#? 1,7"), 35743);
    assert_eq!(part2("?????.??#??##??.?.. 4,7"), 39366);
    assert_eq!(part2("????.?????#???#?# 1,1,12"), 768);
    assert_eq!(part2("?.????#???? 4,2"), 52656);
    assert_eq!(part2("?#??????????#### 5,1,6"), 19029);
    assert_eq!(part2("#??.??..##.???????#? 3,1,2,1,6"), 24576);
}

#[test]
fn part2_10s() {
    assert_eq!(part2("??????.?##??????#? 2,3,9"), 81);
    assert_eq!(part2("?.??????..#??? 1,5,1"), 8192);
    assert_eq!(part2("#???????..????#??? 3,1,2,7"), 1250);
    assert_eq!(part2("?#??????..#?. 3,1,1"), 215408);
    assert_eq!(part2("#??#?#????#???.?? 1,6,2,1,1"), 32768);
    assert_eq!(part2("?#?????#??? 2,1,1"), 267936);
    assert_eq!(part2("???.???#?? 1,4,1"), 54135);
    assert_eq!(part2("##????????#?#?????? 4,1,8,2"), 15224);
    assert_eq!(part2("?#???##????????. 1,5,3"), 102369);
    assert_eq!(part2("???????##??. 1,2,3,1"), 23127);
    assert_eq!(part2("???????..???#??. 5,1,4"), 171366);
    assert_eq!(part2("?#??.?.?#????? 2,1,4"), 117128);
    assert_eq!(part2("?.?.?#?#????. 1,5,1"), 258006);
    assert_eq!(part2("????#?.??#??.? 1,1,4,1"), 124416);
    assert_eq!(part2(".?#??????.?#####? 3,1,6"), 537824);
    assert_eq!(part2("?#?????#?#??.??? 2,7,2"), 207360);
    assert_eq!(part2("???..?????? 3,1,2"), 138350);
    assert_eq!(part2("?????##?###.??. 1,6,1"), 524288);
    assert_eq!(part2("????????##?. 2,2,3"), 589824);
    assert_eq!(part2("???????..??#?. 3,1"), 441488);
    assert_eq!(part2(".???????#??? 2,4"), 1086848);
}

#[test]
fn part2_100s() {
    assert_eq!(part2("?#?#??.????..??? 1,1,3,2"), 514256);
    assert_eq!(part2("?#..????????????? 2,2,7"), 250828);
    assert_eq!(part2(".?###???????.?## 4,2,3"), 944784);
    assert_eq!(part2("?????.??????##. 2,3,3"), 464480);
    assert_eq!(part2("???#??#?.???????? 7,4,1"), 786432);
    assert_eq!(part2("..???????? 2,2"), 2191626);
    assert_eq!(part2("??.??.??#??# 1,1,2,1"), 911370);
    assert_eq!(part2(".??#?????? 3,1,1"), 2185261);
    assert_eq!(part2("#???????#????.?#??? 1,1,1,7,1,1"), 32);
    assert_eq!(part2("????#???#?##.????? 4,4,1"), 3168615);
    assert_eq!(part2("???.?#???????## 1,4,1,2"), 759375);
    assert_eq!(part2("???#?#.???.????.? 4,3,1,1"), 4472217);
    assert_eq!(part2("?.???#?????#???#.??? 1,7,2,1,1,1"), 39366);
    assert_eq!(part2("??.???#???? 1,4,1"), 5595385);
    assert_eq!(part2("???.?#?????? 1,1,1,3"), 671370);
    assert_eq!(part2(".??????.??#?????? 1,1"), 5907426);
    assert_eq!(part2("???????#??.?.? 1,4"), 3746328);
    assert_eq!(part2("?##?.?????.??? 3,3,1"), 10126400);
    assert_eq!(part2(".?.???.??#???...???? 2,1"), 8295505);
    assert_eq!(part2("..?.????#????? 3,2,1"), 2451664);
    assert_eq!(part2("????.?.??.?#?#?.??? 3,1,1,1,2,2"), 2968544);
    assert_eq!(part2("??????????. 4,1"), 17668660);
    assert_eq!(part2("#????????.??.?##..## 1,2,1,1,3,2"), 7962624);
}

#[test]
fn part2_1000s() {
    assert_eq!(part2("??..??????# 1,1,1"), 15545896);
    assert_eq!(part2("??????.?#??? 2,1,3"), 8559632);
    assert_eq!(part2("?#.????..?????#. 1,1,1,1,3"), 1505418);
    assert_eq!(part2("?????#????#?????##?? 1,1,11"), 2514693);
    assert_eq!(part2(".?????.???? 1,1,3"), 11034504);
    assert_eq!(part2("??.#???.????? 1,3,1,1"), 1920000);
    assert_eq!(part2("????.#??.???#?? 2,1,1,1,3"), 5822433);
    assert_eq!(part2("????##?????##?????. 2,3,2,4"), 4741446);
    assert_eq!(part2("???????#???# 1,1,1,1"), 18600576);
    assert_eq!(part2("??#??.??????.?. 3,1,2,1,1"), 3488128);
    assert_eq!(part2("?????##?#??#????. 1,9,1"), 18995083);
    assert_eq!(part2("?.??.?????? 1,1,3"), 38782464);
    assert_eq!(part2(".??????????????#??? 1,7,5,1"), 705862);
    assert_eq!(part2(".??????##??.??#??.? 5,3,1,2"), 380056);
    assert_eq!(part2("?.??.?.?????##???? 1,8"), 31704464);
    assert_eq!(part2("?.??.??.??? 1,1,2"), 92448096);
    assert_eq!(part2(".???.??.#?????#?# 1,1,4,1,1"), 307447);
    assert_eq!(part2("???.????##?.??? 1,2,3"), 74331567);
}

#[test]
fn part2_5000s() {
    assert_eq!(part2("?????.?#.?#????#??? 1,2,7,1"), 50728980);
    assert_eq!(part2("??.?.?##?????? 1,3,1"), 182660427);
    assert_eq!(part2("#??.??.?????? 1,2,1,1"), 82541624);
    assert_eq!(part2("???.?.?.?? 1,1,1"), 452335496);
    assert_eq!(part2("?..????????.? 2,1"), 489355045);
    assert_eq!(part2("????#?#??????#??? 1,3,1,1,4"), 32692514);
}

//
// See day12.txt, which has answers that I trust for a bunch of input lines.
//
// #[test]
// fn test_part2_line16() {
//     // I don't know the actual answer here, but I do know
//     // that it takes a very long time, and the answer must
//     // be very large.
//     assert_eq!(part2("..?.????#?????????? 1,1,1,1,1,4"), 0);
// }
