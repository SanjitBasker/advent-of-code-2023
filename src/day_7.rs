use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("../inputs/day-07/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-07/input_2.txt")));
}

fn part1(input: &str) -> usize {
    let cards = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let mut hand_infos = input
        .split('\n')
        .map(|x| {
            let hand = &x[..x.find(' ').unwrap()];
            let bid = &x[x.find(' ').unwrap() + 1..];
            let mut counts: HashMap<char, u64> = HashMap::new();
            for c in hand.chars() {
                match counts.entry(c) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(1);
                    }
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        e.insert(e.get() + 1);
                    }
                }
            }
            let rank: usize = match counts.values().sorted().collect::<Vec<_>>()[..] {
                [5] => 7,
                [1, 4] => 6,
                [2, 3] => 5,
                [1, 1, 3] => 4,
                [1, 2, 2] => 3,
                [1, 1, 1, 2] => 2,
                [1, 1, 1, 1, 1] => 1,
                _ => {
                    println!(
                        "stuck on {} {:?}",
                        hand,
                        counts.values().sorted().collect::<Vec<_>>()
                    );
                    0
                }
            };
            let sort_keys = hand
                .chars()
                .map(|c| cards.iter().position(|&d| d == c).unwrap())
                .collect::<Vec<_>>();
            (rank, sort_keys, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    hand_infos.sort_by(|h1, h2| match h1.0.cmp(&h2.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            h1.1.iter()
                .zip(h2.1.iter())
                .fold(Ordering::Equal, |cmp, (&a, &b)| {
                    if cmp != Ordering::Equal {
                        cmp
                    } else if a < b {
                        Ordering::Less
                    } else if a > b {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
        }
    });
    let total = hand_infos
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, _, bid))| acc + (idx + 1) * bid);
    total
}

fn part2(input: &str) -> usize {
    let cards = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let mut hand_infos = input
        .split('\n')
        .map(|x| {
            let hand = &x[..x.find(' ').unwrap()];
            let bid = &x[x.find(' ').unwrap() + 1..];
            let mut counts: HashMap<char, u64> = HashMap::new();
            for c in hand.chars() {
                if !counts.contains_key(&c) {
                    counts.insert(c, 1);
                } else {
                    counts.insert(c, counts.get(&c).unwrap() + 1);
                }
            }
            let &jacks = counts.get(&'J').unwrap_or(&0);
            let rank: usize = match counts.values().sorted().collect::<Vec<_>>()[..] {
                [5] => 7,
                [1, 4] => {
                    if jacks == 0 {
                        6
                    } else {
                        7
                    }
                }
                [2, 3] => {
                    if jacks == 0 {
                        5
                    } else {
                        7
                    }
                }
                [1, 1, 3] => {
                    if jacks == 0 {
                        4
                    } else {
                        6
                    }
                }
                [1, 2, 2] => {
                    if jacks == 0 {
                        3
                    } else if jacks == 2 {
                        6
                    } else {
                        5
                    }
                }
                [1, 1, 1, 2] => {
                    if jacks == 0 {
                        2
                    } else {
                        4
                    }
                }
                [1, 1, 1, 1, 1] => {
                    if jacks == 0 {
                        1
                    } else {
                        2
                    }
                }
                _ => {
                    println!(
                        "stuck on {} {:?}",
                        hand,
                        counts.values().sorted().collect::<Vec<_>>()
                    );
                    0
                }
            };
            let sort_keys = hand
                .chars()
                .map(|c| cards.iter().position(|&d| d == c).unwrap())
                .collect::<Vec<_>>();
            (rank, sort_keys, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    hand_infos.sort_by(|h1, h2| match h1.0.cmp(&h2.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            h1.1.iter()
                .zip(h2.1.iter())
                .fold(Ordering::Equal, |cmp, (&a, &b)| {
                    if cmp != Ordering::Equal {
                        cmp
                    } else if a < b {
                        Ordering::Less
                    } else if a > b {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
        }
    });
    let total = hand_infos
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, _, bid))| acc + (idx + 1) * bid);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-07/input_1.txt")) == 6440);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-07/input_1.txt")) == 5905);
    }
}
