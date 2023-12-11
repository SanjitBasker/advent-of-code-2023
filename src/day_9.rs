fn main() {
    println!("{}", part1(include_str!("../inputs/day-09/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-09/input_2.txt")));
}

fn diff(v: &Vec<i64>) -> Vec<i64> {
    let mut ans = vec![];
    for i in 0..v.len() - 1 {
        ans.push(v[i + 1] - v[i]);
    }
    ans
}

fn part1(input: &str) -> i64 {
    let lines = input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|piece| piece.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    lines
        .iter()
        .map(|seq| {
            let mut subseqs = vec![];
            subseqs.push(seq.clone());
            while subseqs.last().unwrap().iter().any(|x| *x != 0) {
                subseqs.push(diff(subseqs.last().unwrap()));
            }
            println!("{}", subseqs.len());
            subseqs.reverse();
            let ans = subseqs
                .iter()
                .skip(1)
                .inspect(|_l| {})
                .fold(0, |acc, lst| acc + lst.last().unwrap());
            ans
        })
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let lines = input
        .split('\n')
        .map(|line| {
            let mut ans = line
                .split(' ')
                .map(|piece| piece.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            ans.reverse();
            ans
        })
        .collect::<Vec<_>>();
    lines
        .iter()
        .map(|seq| {
            let mut subseqs = vec![];
            subseqs.push(seq.clone());
            while subseqs.last().unwrap().iter().any(|x| *x != 0) {
                subseqs.push(diff(subseqs.last().unwrap()));
            }
            subseqs.reverse();
            let ans = subseqs
                .iter()
                .skip(1)
                .inspect(|_l| {})
                .fold(0, |acc, lst| acc + lst.last().unwrap());
            ans
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-09/input_1.txt")) == 114);
        assert!(part1(include_str!("../inputs/day-09/input_2.txt")) == 1887980197);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-09/input_1.txt")) == 2);
        assert!(part2(include_str!("../inputs/day-09/input_2.txt")) == 990);
    }
}
