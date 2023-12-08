use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 253205868);
}

fn part1(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::from_str).collect_vec();
    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| (i as u32 +1) * hand.bid).sum()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    kind: HandKind,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        let (cards_str, bid_str) = s.split_once(' ').expect("a space");
        let cards: Vec<u32> = cards_str
            .chars()
            .map(|c| match c {
                d if c.is_ascii_digit() => d.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("unknown card"),
            })
            .collect();
        assert_eq!(cards.len(), 5);
        let bid = bid_str.parse().expect("valid bid");

        // Figure out the kind of hand.
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_unstable();
        let mut groups: Vec<_> = sorted_cards.into_iter().group_by(|card| *card).into_iter().map(|(_key, group)| group.count()).collect();
        groups.sort_unstable();
        let kind = if groups == vec![5] {
            HandKind::FiveOfAKind
        } else if groups == vec![1, 4] {
            HandKind::FourOfAKind
        } else if groups == vec![2, 3] {
            HandKind::FullHouse
        } else if groups == vec![1, 1, 3] {
            HandKind::ThreeOfAKind
        } else if groups == vec![1, 2, 2] {
            HandKind::TwoPair
        } else if groups == vec![1, 1, 1, 2] {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        };

        Hand { cards, bid, kind }
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut result = self.kind.cmp(&other.kind);
        if result == std::cmp::Ordering::Equal {
            result = self.cards.cmp(&other.cards);
        }
        result
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
static EXAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 6440);
}
