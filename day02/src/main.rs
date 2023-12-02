use std::{str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 2006);
}

fn part1(input: &str) -> u32 {
    let games = parse_games(input);
    games.iter().filter_map(|game| {
        for d in game.draw.iter() {
            if d.red > 12 || d.green > 13 || d.blue > 14 {
                return None;
            }
        }
        Some(game.id)
    })
    .sum()
}

#[derive(PartialEq, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}
impl Cubes {
    fn new() -> Cubes {
        Cubes{red:0, green:0, blue:0}
    }
}
#[derive(PartialEq, Debug, Default)]
struct Game {
    id: u32,
    draw: Vec<Cubes>
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, cube_list) = s.split_once(": ").expect("invalid input");
        let id = game_id[5..].parse().expect("invalid game ID");
        let draw = cube_list.split("; ").map(|s| {
            let mut cubes = Cubes::new();
            for cube in s.split(", ") {
                let (num, color) = cube.split_once(' ').expect("invalid input");
                let num = num.parse().expect("invalid cube count");
                match color {
                    "red" => cubes.red = num,
                    "green" => cubes.green = num,
                    "blue" => cubes.blue = num,
                    _ => panic!("invalid color")
                }
            }
            cubes
        }).collect();
        Ok(Game{id, draw})
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(|line| line.parse().expect("invalid input")).collect()
}

#[test]
fn test_parse_games() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(parse_games(input), vec![
        Game{id: 1, draw: vec![
            Cubes{blue:3, red:4, green:0},
            Cubes{red:1, green:2, blue:6},
            Cubes{green:2, red:0, blue:0}
        ]},
        Game{id: 2, draw: vec![
            Cubes{blue:1, green:2, red:0},
            Cubes{green:3, blue:4, red:1},
            Cubes{green:1, blue:1, red:0}
        ]},
        Game{id: 3, draw: vec![
            Cubes{green:8, blue:6, red:20},
            Cubes{blue:5, red:4, green:13},
            Cubes{green:5, red:1, blue:0}
        ]},
        Game{id: 4, draw: vec![
            Cubes{green:1, red:3, blue:6},
            Cubes{green:3, red:6, blue:0},
            Cubes{green:3, blue:15, red:14}
        ]},
        Game{id: 5, draw: vec![
            Cubes{red:6, blue:1, green:3},
            Cubes{blue:2, red:1, green:2}
        ]}
    ]);
}

#[test]
fn example1() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(part1(input), 8);
}
