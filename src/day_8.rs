use num::integer;
use std::collections::HashMap;

fn main() {
    println!("{}", part1(include_str!("../inputs/day-08/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-08/input_2.txt")));
}

fn part1(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let turns = lines[0].to_string();
    let mut left: HashMap<String, String> = HashMap::new();
    let mut right: HashMap<String, String> = HashMap::new();
    for line in lines.iter().skip(2) {
        left.insert(line[..3].to_string(), line[7..10].to_string());
        right.insert(line[..3].to_string(), line[12..15].to_string());
    }

    let mut pos = "AAA".to_string();
    let mut count = 0;
    let mut i = 0;
    let el = "L".to_string();
    while pos != *"ZZZ" {
        if turns[i..i + 1] == el {
            pos = left[&pos].to_string();
        } else {
            pos = right[&pos].to_string();
        }
        count += 1;
        i += 1;
        i %= turns.len();
    }
    count
}

fn part2(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let turns = lines[0];
    let mut left: HashMap<String, String> = HashMap::new();
    let mut right: HashMap<String, String> = HashMap::new();
    for line in lines.iter().skip(2) {
        left.insert(line[..3].to_string(), line[7..10].to_string());
        right.insert(line[..3].to_string(), line[12..15].to_string());
    }

    let positions = left
        .keys()
        .filter_map(|k| {
            if k.ends_with('A') {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let counts = positions.iter().map(|pos| {
        let mut curr = pos.clone();
        let mut count = 0;
        let mut i = 0;
        let el = "L".to_string();
        while !curr.ends_with('Z') {
            if turns[i..i + 1] == el {
                curr = left[&curr].to_string();
            } else {
                curr = right[&curr].to_string();
            }
            count += 1;
            i += 1;
            i %= turns.len();
        }
        count
    });
    counts.fold(1, integer::lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-08/input_1.txt")) == 6);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-08/input_3.txt")) == 6);
    }
}
