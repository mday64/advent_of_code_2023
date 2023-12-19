use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 37099); // too low
}

fn part1(input: &str) -> i32 {
    let mut lagoon: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    lagoon.insert((x, y));

    for line in input.lines() {
        let mut fields = line.split(' ');
        let direction = fields.next().unwrap();
        let distance = fields.next().unwrap().parse::<i32>().unwrap();
        let (dx, dy) = match direction {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            d => panic!("Unknown direction: {d}")
        };
        for _ in 0..distance {
            x += dx;
            y += dy;
            lagoon.insert((x, y));
        }
    }

    // Find the vertical bounds of the exterior
    let mut min_y = 0;
    let mut max_y = 0;
    for &(_x, y) in lagoon.iter() {
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    //
    // This is wrong.  Consider a shape like:
    //
    // #######.....
    // #.....#.....
    // #.....#.....
    // ###...#.###. <-- 4
    // ..##..###.## <-- 5
    // ...#.......#
    // ...#.......#
    // ...#########
    //
    // On line 4, the first group of dots are interior, but the rest are
    // exterior.  On line 5, the first two groups of dots are interior,
    // but the last is exterior.  That's because there is a local minima
    // or maxima on that line, so the horizontal part doesn't indicate
    // a switch between interior and exterior.
    //
    // This feels like Day 10, Part 2.  Can we solve this with the
    // "horizontal line test"?  That is, can we determine interior vs.
    // exterior by keeping track of whether a horizontal line extends
    // above and/or below a horizontal line?
    //

    let mut area = 0;
    for y in min_y..=max_y {
        let mut xs = lagoon.iter().filter_map(|&(x0,y0)| (y==y0).then_some(x0)).collect::<Vec<_>>();
        xs.sort_unstable();
        area += 1;
        let mut x = xs[0];
        let mut interior = false;
        for &x2 in &xs[1..] {
            if x2 == x + 1 {
                area += 1;
            } else {
                interior = !interior;
                if interior {
                    area += x2 - x;
                } else {
                    area += 1;
                }
            }
            x = x2;
        }
    }

    area
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
