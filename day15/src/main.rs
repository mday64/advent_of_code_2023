fn main() {
    let input = include_str!("../input.txt").trim_end();

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 517551);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 286097);
}

fn hash_str(word: &str) -> u32 {
    word.chars()
        .fold(0, |acc, elem| ((acc + elem as u32) * 17) & 0xFF)
}

fn part1(input: &str) -> u32 {
    input.split(',').map(hash_str).sum()
}

fn part2(input: &str) -> u32 {
    // Create our empty hash table
    let mut hashmap: Vec<Vec<BucketItem>> = Vec::new();
    for _ in 0..256 {
        hashmap.push(Vec::new());
    }

    // Insert our items into the hashmap
    for word in input.split(',') {
        if let Some((label, focal_length)) = word.split_once('=') {
            let focal_length: u32 = focal_length.parse().unwrap();
            let bucket_index = hash_str(label) as usize;
            let mut found = false;
            for item in hashmap[bucket_index].iter_mut() {
                if item.label == label {
                    item.focal_length = focal_length;
                    found = true;
                }
            }
            if !found {
                hashmap[bucket_index].push(BucketItem {
                    label,
                    focal_length,
                });
            }
        } else if let Some((label, empty)) = word.split_once('-') {
            assert!(empty.is_empty());
            let bucket_index = hash_str(label) as usize;
            hashmap[bucket_index].retain(|item| item.label != label);
        } else {
            panic!("syntax error");
        }
    }

    // Computing the result
    // for all (box, label, focal_length):
    //      (box+1) * index_of(label)_in_box * focal_length
    // .sum()
    hashmap
        .into_iter()
        .enumerate()
        .map(|(box_num, bucket)| {
            bucket
                .into_iter()
                .enumerate()
                .map(|(i, item)| (box_num as u32 + 1) * (i as u32 + 1) * item.focal_length)
                .sum::<u32>()
        })
        .sum()
}

struct BucketItem<'label> {
    label: &'label str,
    focal_length: u32,
}

#[cfg(test)]
static EXAMPLE1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE1), 1320);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE1), 145);
}
