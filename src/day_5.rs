use std::cmp;

use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("../inputs/day-05/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-05/input_2.txt")));
    // println!("{}", part2(include_str!("../inputs/day-04/input_2.txt")));
}

fn part1(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let colon_index = lines[0].find(": ").unwrap();
    let seeds = lines[0][colon_index + 2..]
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let binding = lines[1..]
        .iter()
        .group_by(|&&line| line.is_empty() || line.contains(':'));
    let map_lines = binding
        .into_iter()
        .filter_map(|(is_space, group)| {
            if !is_space {
                Some(group.collect::<Vec<_>>())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let maps = map_lines
        .iter()
        .map(|map_lines| {
            map_lines
                .iter()
                .map(|&map_line| {
                    map_line
                        .split(' ')
                        .map(|piece| piece.parse::<u64>().unwrap())
                        .collect_tuple::<(u64, u64, u64)>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let seed_dests = seeds
        .iter()
        .map(|&seed| {
            let outer_ans = maps.iter().fold(seed, |loc, map| {
                let ans = map
                    .iter()
                    .fold(loc, |curr_loc, (dest_start, source_start, len)| {
                        if loc == curr_loc
                            && *source_start <= curr_loc
                            && curr_loc < *source_start + len
                        {
                            dest_start + (curr_loc - *source_start)
                        } else {
                            curr_loc
                        }
                    });
                ans
            });
            outer_ans
        })
        .collect::<Vec<_>>();
    *seed_dests.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let colon_index = lines[0].find(": ").unwrap();
    let seeds = lines[0][colon_index + 2..]
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let binding = lines[1..]
        .iter()
        .group_by(|&&line| line.is_empty() || line.contains(':'));
    let map_lines = binding
        .into_iter()
        .filter_map(|(is_space, group)| {
            if !is_space {
                Some(group.collect::<Vec<_>>())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let maps = map_lines
        .iter()
        .map(|map_lines| {
            map_lines
                .iter()
                .map(|&map_line| {
                    map_line
                        .split(' ')
                        .map(|piece| piece.parse::<u64>().unwrap())
                        .collect_tuple::<(u64, u64, u64)>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let apply_maps = |seed| {
        let mut min_slack = u64::MAX - seed;
        let ans = maps.iter().fold(seed, |loc, map| {
            map.iter()
                .fold(loc, |curr_loc, (dest_start, source_start, len)| {
                    if loc == curr_loc
                        && *source_start <= curr_loc
                        && curr_loc < *source_start + len
                    {
                        min_slack = cmp::min(min_slack, len - (curr_loc - *source_start));
                        dest_start + curr_loc - *source_start
                    } else {
                        curr_loc
                    }
                })
        });
        (ans, min_slack)
    };
    let binding = seeds.iter().chunks(2);
    let pairs = binding
        .into_iter()
        .map(|x| x.into_iter().collect_tuple::<(&u64, &u64)>().unwrap())
        .collect::<Vec<_>>();

    pairs
        .iter()
        .map(|(&a, &b)| {
            let mut curr = a;
            let mut best = u64::MAX;
            while curr < a + b {
                let (dest, slack) = apply_maps(curr);
                curr += slack;
                best = cmp::min(best, dest);
            }
            best
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-05/input_1.txt")) == 35);
        assert!(part1(include_str!("../inputs/day-05/input_2.txt")) == 382895070);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-05/input_1.txt")) == 46);
        assert!(part2(include_str!("../inputs/day-05/input_2.txt")) == 17729182);
    }
}
