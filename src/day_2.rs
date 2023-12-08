use std::cmp;
use std::convert;

fn main() {
    println!(
        "{}",
        part1(include_str!("../inputs/day-02/input_2.txt"), 12, 13, 14)
    );
    println!("{}", part2(include_str!("../inputs/day-02/input_2.txt")));
}

fn part1(input: &str, r: u32, g: u32, b: u32) -> u32 {
    let nums = input.split('\n').map(|line| {
        let two_parts = line.split(": ").collect::<Vec<_>>();
        let header = two_parts[0];
        let id = header[header.find(' ').unwrap() + 1..(header.len())].to_string();

        let item_list = two_parts[1]
            .split("; ")
            .map(|list| list.split(", ").collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let each_satisfies = item_list
            .iter()
            .map(|parts| {
                parts
                    .iter()
                    .map(|part| {
                        let space = part.find(' ').unwrap();
                        let quantity = part[0..space].parse::<u32>().unwrap();
                        let color = part[space + 1..].to_string();
                        quantity
                            <= (if color == *"red" {
                                r
                            } else if color == *"green" {
                                g
                            } else {
                                b
                            })
                    })
                    .all(convert::identity)
            })
            .all(convert::identity);
        if each_satisfies {
            id.parse::<u32>().unwrap()
        } else {
            0
        }
    });
    nums.sum()
}

fn part2(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| {
            let two_parts = line.split(": ").collect::<Vec<_>>();
            let header = two_parts[0];
            let _id = header[header.find(' ').unwrap() + 1..(header.len())].to_string();

            let item_list = two_parts[1]
                .split("; ")
                .map(|list| list.split(", ").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            item_list
                .iter()
                .map(|parts| {
                    parts
                        .iter()
                        .map(|part| {
                            let space = part.find(' ').unwrap();
                            let quantity = part[0..space].parse::<u32>().unwrap();
                            let color = part[space + 1..].to_string();

                            if color == *"red" {
                                (quantity, 0, 0)
                            } else if color == *"green" {
                                (0, quantity, 0)
                            } else {
                                (0, 0, quantity)
                            }
                        })
                        .fold((0, 0, 0), |a, b| {
                            (cmp::max(a.0, b.0), cmp::max(a.1, b.1), cmp::max(a.2, b.2))
                        })
                })
                .fold((0, 0, 0), |a, b| {
                    (cmp::max(a.0, b.0), cmp::max(a.1, b.1), cmp::max(a.2, b.2))
                })
        })
        .map(|triple| triple.0 * triple.1 * triple.2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-02/input_1.txt"), 12, 13, 14) == 8);
        assert!(part1(include_str!("../inputs/day-02/input_2.txt"), 12, 13, 14) == 2278);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-02/input_1.txt")) == 2286);
        assert!(part2(include_str!("../inputs/day-02/input_2.txt")) == 67953);
    }
}
