use std::ops::Range;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 39039);
}

fn part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut perimeter = 0;
    let mut vertical_lines = vec![];
    let mut horizontal_lines = vec![];

    for line in input.lines() {
        let mut fields = line.split(' ');
        let direction = fields.next().unwrap();
        let distance = fields.next().unwrap().parse::<i32>().unwrap();
        perimeter += distance;
        match direction {
            "U" => {
                vertical_lines.push(LineSegment{ends: (y-distance)..y, mid: x});
                y -= distance;
            }
            "D" => {
                vertical_lines.push(LineSegment{ends: y..(y+distance), mid: x});
                y += distance;
            }
            "L" => {
                horizontal_lines.push(LineSegment{ends: (x-distance)..x, mid: y});
                x -= distance;
            }
            "R" => {
                horizontal_lines.push(LineSegment{ends: x..(x+distance), mid: y});
                x += distance;
            }
            d => panic!("Unknown direction: {d}")
        }
    }

    // Sort the vertical lines so we can traverse them from left to right.
    vertical_lines.sort_unstable_by_key(|line| line.mid);

    // Find the vertical bounds of the pit
    let (top, bottom) = horizontal_lines.iter().map(|line| line.mid).minmax().into_option().unwrap();

    // Compute the interior area
    let mut interior = 0;
    for y in (top+1)..bottom {
        for (left,right) in vertical_lines
            .iter()
            .filter(|line| line.ends.contains(&y))
            .tuples()
        {
            interior += right.mid - left.mid - 1;

            // Subtract off horizontal lines that intersect
            let h_range = left.mid..right.mid;
            for line in horizontal_lines.iter().filter(|line| line.mid == y) {
                interior -= h_range.intersect(&line.ends).count() as i32;
            }
        }
    }

    perimeter + interior
}

/// A horizontal or vertical line segment
struct LineSegment {
    ends: Range<i32>,   // Y for vertical, X for horizontal
    mid: i32            // X for vertixal, Y for horizontal
}

trait RangeIntersect {
    fn intersect(&self, other: &Self) -> Self;
}

impl<T: Ord + Copy> RangeIntersect for Range<T> {
    fn intersect(&self, other: &Self) -> Self {
        self.start.max(other.start) .. self.end.min(other.end)
    }
}

#[cfg(test)]
static EXAMPLE1: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 62);
}

#[cfg(test)]
static MY_EXAMPLE: &str = "\
R 6 (#70c710)
D 4 (#0dc571)
R 2 (#5713f0)
U 1 (#d2c081)
R 2 (#59c680)
D 1 (#411b91)
R 1 (#8ceee2)
D 3 (#caa173)
L 8 (#1b58a2)
U 3 (#caa171)
L 1 (#7807d2)
U 1 (#a77fa3)
L 2 (#015232)
U 3 (#7a21e3)
";

#[test]
fn test_part1_my_example() {
    assert_eq!(part1(MY_EXAMPLE), 68);
}
