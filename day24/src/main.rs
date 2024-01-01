use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let hailstones = parse_input(input);

    let result1 = part1(&hailstones, 200000000000000, 400000000000000);
    println!("Part 1: {result1}");
    assert_eq!(result1, 20847);     // 10826 is too low?
}

fn part1(hailstones: &[Hailstone], position_min: i64, position_max: i64) -> usize {
    let mut result = 0;

    for pair in hailstones.iter().combinations(2) {
        let x1 = pair[0].x;
        let y1 = pair[0].y;
        let x2 = pair[0].x + pair[0].vx;
        let y2 = pair[0].y + pair[0].vy;
        let x3 = pair[1].x;
        let y3 = pair[1].y;
        let x4 = pair[1].x + pair[1].vx;
        let y4 = pair[1].y + pair[1].vy;

        // let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let denom = pair[0].vx * pair[1].vy - pair[0].vy * pair[1].vx;
        if denom == 0 {
            // Lines are parallel or coincident.
            // We need to do more checks to see if they are coincident.
            // If so, we need to see if their future positions land in
            // the target area.
            // dbg!(&pair);

            // If the lines are coincident, the difference in starting
            // coordinates will be a multiple of both velocity vectors.
            // That multiple will be the same for both X and Y.
            let dx = pair[1].x - pair[0].x;
            let dy = pair[1].y - pair[0].y;
            // Check if dx/vx == dy/vy; equivaently, dx*vy == dy*vx
            if dx * pair[0].vy != dy * pair[0].vx {
                // must be parallel
                continue;
            }

            // Figure out whether both half-lines pass through the target
            // area in the future.  Figure out the range of time that the
            // X and Y coordinates are in the target range.  Find that
            // intersection to see when (if) the hailstone is in the target
            // area.  Make sure the end of that time range is not in the past.
            // t = (edge_of_range - initial_position) / velocity
            panic!("Lines are coincident and may intersect");
        }

        let t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);

        // Ignore positions "in the past"
        if denom > 0 {
            if t_num < 0 || u_num < 0 {
                continue;
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if t_num > 0 || u_num > 0 {
                continue;
            }
        }

        //
        // To find the actual intersection point, we need to know:
        //      t = t_num / denom
        //      u = u_num / denom
        // and then substitute into either:
        // x = x1 + t*(x2-x1), y = y1 + t*(y2-y1), or:
        // x = x3 + u*(x4-x3), y = y3 + u*(y4-y3)
        // (Note: x2-x1, y2-y1, x4-x3, y4-y3 are pair[0].vx, pair[0].vy,
        // pair[1].vx, pair[1].xy, respectively).
        //
        // ... and then make sure the resulting x and y are within
        // position_min ..= position_max.
        //
        // If we multiply everything by denom, we can avoid division
        // (at the risk of integer overflow).
        let x = x1 as f64 + t_num as f64 / denom as f64 * (x2 - x1) as f64;
        let y = y1 as f64 + t_num as f64 / denom as f64 * (y2 - y1) as f64;
        if x >= position_min as f64
            && x <= position_max as f64
            && y >= position_min as f64
            && y <= position_max as f64
        {
            result += 1;
        }
    }

    result
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();
            let (x, y, z) = position
                .split(", ")
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (vx, vy, vz) = velocity
                .split(", ")
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            Hailstone {
                x,
                y,
                z,
                vx,
                vy,
                vz,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Hailstone {
    x: i64,
    y: i64,
    #[allow(dead_code)]
    z: i64,
    vx: i64,
    vy: i64,
    #[allow(dead_code)]
    vz: i64,
}

//
// See <https://paulbourke.net/geometry/pointlineplane/>
// and <https://math.stackexchange.com/questions/25171/intersection-of-two-lines-in-2d>
// and <https://en.wikipedia.org/wiki/Line–line_intersection>
// and <https://mikespivey.wordpress.com/2016/10/06/how-do-you-tell-whether-two-lines-intersect-in-3d/>
// and <https://stackoverflow.com/questions/2316490/the-algorithm-to-find-the-point-of-intersection-of-two-3d-line-segment>
//

#[cfg(test)]
static EXAMPLE1: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

#[test]
fn test_part1() {
    let hailstones = parse_input(EXAMPLE1);
    assert_eq!(part1(&hailstones, 7, 27), 2);
}
