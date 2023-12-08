fn main() {
    println!("{}", part1(include_str!("../inputs/day-01/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-01/input_2.txt")));
}

fn part1(input: &str) -> String {
    input
        .split('\n')
        .map(|part| {
            let nums = part
                .chars()
                .filter(|c| (&'0'..=&'9').contains(&c))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let d1 = nums.first();
            let d2 = nums.last();
            d1.unwrap_or(&0) * 10 + d2.unwrap_or(&0)
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .to_string()
        .split('\n')
        .map(|part| {
            let digits = vec![
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];

            let mut i = 0;
            let mut ans = vec![];
            while i < part.len() {
                let mut pushed = false;
                for (s, d) in &digits {
                    if i + s.len() <= part.len() && part[i..i + s.len()] == **s {
                        i += 1;
                        ans.push(d.to_string());
                        pushed = true;
                        break;
                    }
                }
                if !pushed {
                    ans.push(part[i..i + 1].to_string());
                    i += 1;
                }
            }

            let next_part = ans.join("");

            let nums = next_part
                .chars()
                .filter(|c| (&'0'..=&'9').contains(&c))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let d1 = nums.first();
            let d2 = nums.last();
            d1.unwrap_or(&0) * 10 + d2.unwrap_or(&0)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-01/input_1.txt")) == "142")
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-01/input_3.txt")) == "281")
    }
}
