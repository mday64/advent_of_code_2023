use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 559667);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 86841457);
}

fn part1(input: &str) -> u32 {
    // Find the location of all symbols
    let mut symbols = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' && !ch.is_ascii_digit() {
                symbols.insert((row as i32, col as i32));
            }
        }
    }

    let mut total = 0;
    let mut current_number = 0;

    // Find every number, including its starting and ending position.
    for (row, line) in input.lines().enumerate() {
        let mut start_col = None;
        // We need to be sure to handle numbers at the end of a line.
        // The easiest way is to pretend there is an extra '.' at the end
        // of every line.
        for (col, ch) in line.chars().chain(Some('.')).enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                    if start_col.is_none() {
                        start_col = Some(col as i32);
                        current_number = 0;
                    }
                    current_number = current_number * 10 + digit;
            } else {
                if start_col.is_some() {
                    // Finished parsing a number.  Look for adjacent symbols
                    let row = row as i32;
                    let start_col = start_col.unwrap();
                    let end_col = col as i32;
                    let mut has_adjacent_symbol = false;
                    for r in row-1 ..= row+1 {
                        for c in start_col-1 ..= end_col {
                            if symbols.contains(&(r,c)) {
                                has_adjacent_symbol = true;
                            }
                        }
                    }

                    if has_adjacent_symbol {
                        total += current_number;
                    }
                }
                start_col = None;
            }
        }
    }

    total
}

fn part2(input: &str) -> u32 {
    // Find the location of all '*' symbols.  The value in the map is the
    // list of adjacent numbers.
    let mut gears = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                gears.insert((row as i32, col as i32), vec![]);
            }
        }
    }

    let mut current_number = 0;

    // Find every number, including its starting and ending position.
    for (row, line) in input.lines().enumerate() {
        let mut start_col = None;
        // We need to be sure to handle numbers at the end of a line.
        // The easiest way is to pretend there is an extra '.' at the end
        // of every line.
        for (col, ch) in line.chars().chain(Some('.')).enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                    if start_col.is_none() {
                        start_col = Some(col as i32);
                        current_number = 0;
                    }
                    current_number = current_number * 10 + digit;
            } else {
                if start_col.is_some() {
                    // Finished parsing a number.  Look for adjacent '*' symbols.
                    let row = row as i32;
                    let start_col = start_col.unwrap();
                    let end_col = col as i32;
                    for r in row-1 ..= row+1 {
                        for c in start_col-1 ..= end_col {
                            // Append this number to this '*' symbol.
                            if let Some(gear) = gears.get_mut(&(r,c)) {
                                gear.push(current_number);
                            }
                        }
                    }
                }
                start_col = None;
            }
        }
    }

    // Sum the products of all '*' symbols with exactly two numbers.
    gears.values()
    .filter(|v| v.len() == 2)
    .map(|v| v.iter().product::<u32>())
    .sum()
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


#[test]
fn example2() {
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
    assert_eq!(part2(input), 467835);
}
