use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("../inputs/day-06/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-06/input_2.txt")));
}

fn part1(input: &str) -> u64 {
    let (a, b) = input.split_at(input.find('\n').unwrap());
    let first_line = a.replace("Time:", "");
    let second_line = b.replace("\nDistance:", "");
    let times = first_line
        .split_whitespace()
        .map(|piece| piece.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = second_line
        .split_whitespace()
        .map(|piece| piece.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let ans = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| {
            let tf = t as f64;
            let df = d as f64;
            let s = (tf * tf / 4. - df).sqrt();
            let lower = tf / 2. - s;
            let upper = tf / 2. + s;
            upper.ceil() as u64 - lower.floor() as u64 - 1
        })
        .product::<u64>();
    ans
}

fn part2(input: &str) -> u64 {
    let (a, b) = input.split_at(input.find('\n').unwrap());
    let first_line = a.replace("Time:", "");
    let second_line = b.replace("\nDistance:", "");
    let t = first_line
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();
    let d = second_line
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let tf = t as f64;
    let df = d as f64;
    let s = (tf * tf / 4. - df).sqrt();
    let lower = tf / 2. - s;
    let upper = tf / 2. + s;
    upper.ceil() as u64 - lower.floor() as u64 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-06/input_1.txt")) == 288);
        assert!(part1(include_str!("../inputs/day-06/input_2.txt")) == 5133600);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-06/input_1.txt")) == 71503);
        assert!(part2(include_str!("../inputs/day-06/input_2.txt")) == 40651271);
    }
}
