use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 362930);
}

fn part1(input: &str) -> u32 {
    let (rules, items) = input.split_once("\n\n").unwrap();
    let rules: HashMap<&str, Vec<Rule>> = rules.lines().map(|line| {
        let mut parts = line.split(['{', '}']);
        let key = parts.next().unwrap();
        let checks = parts.next().unwrap().split(',').map(|s| {
            if let Some((cond, next)) = s.split_once(':') {
                if let Some((letter, value)) = cond.split_once('<') {
                    Rule{ condition: Some(Condition::LessThan(letter, value.parse().unwrap())), next }
                } else if let Some((letter, value)) = cond.split_once('>') {
                    Rule{ condition: Some(Condition::GreaterThan(letter, value.parse().unwrap())), next }
                } else {
                    panic!("Invalid condition: {cond}");
                }
            } else {
                Rule{ condition: None, next: s }
            }
        })
        .collect();
        (key, checks)
    })
    .collect();
    let items: Vec<HashMap<&str, u32>> = items.lines().map(|line| {
        let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        line.split(',').map(|assignment| {
            let (letter, value) = assignment.split_once('=').unwrap();
            let value = value.parse().unwrap();
            (letter, value)
        }).collect()
    })
    .collect();

    items.into_iter().filter_map(|item| {
        // Process item with the rules => Option<u32>
        let mut rule_name = "in";
        loop {
            if rule_name == "R" {
                return None;
            }
            if rule_name == "A" {
                return Some(item.values().sum::<u32>());
            }
            let current_rules = rules.get(rule_name).unwrap();
            for rule in current_rules {
                match rule.condition {
                    None => {
                        rule_name = rule.next;
                        break;
                    }
                    Some(Condition::LessThan(letter, value)) => {
                        if item.get(letter).unwrap() < &value {
                            rule_name = rule.next;
                            break;
                        }
                    }
                    Some(Condition::GreaterThan(letter, value)) => {
                        if item.get(letter).unwrap() > &value {
                            rule_name = rule.next;
                            break;
                        }
                    }
                }
            }
        }
    }).sum()
}

enum Condition<'a> {
    LessThan(&'a str, u32),
    GreaterThan(&'a str, u32),
}

struct Rule<'a> {
    condition: Option<Condition<'a>>,
    next: &'a str,
}

#[cfg(test)]
static EXAMPLE1: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 19114);
}
