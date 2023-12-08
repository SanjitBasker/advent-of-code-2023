use std::cmp;

use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("../inputs/day-01/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-01/input_2.txt")));
}

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-01/input_1.txt")) == 0);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-01/input_1.txt")) == 0);
    }
}
