fn main() {
    println!("{}", part1(include_str!("../inputs/day-04/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-04/input_2.txt")));
}

fn part2(input: &str) -> usize {
    let matches = input
        .replace("  ", " ")
        .split('\n')
        .map(|line| {
            let a = line.find(':').unwrap() + 2;
            let midpoint = line.find(" | ").unwrap();
            let left = &line[a..midpoint];
            let right = &line[midpoint + 3..];
            let left_nums = left
                .split(' ')
                .map(|piece| piece.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let count_right = right
                .split(' ')
                .map(|piece| piece.parse::<usize>().unwrap())
                .filter(|i| left_nums.contains(i))
                .fold(0, |score, _i| score + 1);
            count_right
        })
        .collect::<Vec<_>>();

    let n = matches.len();

    let mut counts = vec![1; n];
    for i in 0..n {
        for j in i + 1..i + matches[i] + 1 {
            counts[j] += counts[i];
        }
    }
    counts.into_iter().sum::<usize>()
}

fn part1(input: &str) -> u32 {
    input
        .replace("  ", " ")
        .split('\n')
        .map(|line| {
            let a = line.find(':').unwrap() + 2;
            let midpoint = line.find(" | ").unwrap();
            let left = &line[a..midpoint];
            let right = &line[midpoint + 3..];
            let left_nums = left
                .split(' ')
                .map(|piece| piece.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let count_right = right
                .split(' ')
                .map(|piece| piece.parse::<u32>().unwrap())
                .filter(|i| left_nums.contains(i))
                .fold(0, |score, _i| if score == 0 { 1 } else { score * 2 });
            count_right
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-04/input_1.txt")) == 13);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-04/input_1.txt")) == 30);
    }
}
