fn main() {
    println!("{}", part1(include_str!("../inputs/day-03/input_2.txt")));
    println!("{}", part2(include_str!("../inputs/day-03/input_2.txt")));
}

#[derive(Debug, PartialEq)]
enum Cell {
    Num { n: u32 },
    Char { c: char },
    Period,
}

fn part1(input: &str) -> u32 {
    let mut lines = input
        .split('\n')
        .map(|piece| {
            let mut ans = vec![];

            let mut n = 0;
            let mut count = 0;
            for c in piece.chars() {
                if c.is_ascii_digit() {
                    n *= 10;
                    n += c.to_digit(10).unwrap();
                    count += 1;
                } else {
                    for _ in 0..count {
                        ans.push(Cell::Num { n });
                    }
                    count = 0;
                    n = 0;
                    if c == '.' {
                        ans.push(Cell::Period);
                    } else {
                        ans.push(Cell::Char { c });
                    }
                }
            }
            for _ in 0..count {
                ans.push(Cell::Num { n });
            }
            ans
        })
        .collect::<Vec<_>>();
    let n = lines.len();
    let m = lines[0].len();

    let mut ans = 0;

    for i in 0i32..(n.try_into().unwrap()) {
        for j in 0i32..(m.try_into().unwrap()) {
            if let Cell::Char { c: _ } = lines[TryInto::<usize>::try_into(i).unwrap()]
                [TryInto::<usize>::try_into(j).unwrap()]
            {
                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        if ((di == 0) && (dj == 0))
                            || (i + di >= n.try_into().unwrap())
                            || (i + di < 0)
                            || (j + dj >= m.try_into().unwrap())
                            || (j + dj < 0)
                        {
                            continue;
                        }
                        let ei = i + di;
                        let ej = j + dj;
                        let mut begin = ej;
                        if let Cell::Num { n } = lines[TryInto::<usize>::try_into(ei).unwrap()]
                            [TryInto::<usize>::try_into(ej).unwrap()]
                        {
                            while (begin >= 0
                                && lines[TryInto::<usize>::try_into(ei).unwrap()]
                                    [TryInto::<usize>::try_into(begin).unwrap()]
                                    == Cell::Num { n })
                            {
                                begin -= 1;
                            }
                            begin += 1;
                            ans += n;
                            while (begin < m.try_into().unwrap()
                                && lines[TryInto::<usize>::try_into(ei).unwrap()]
                                    [TryInto::<usize>::try_into(begin).unwrap()]
                                    == Cell::Num { n })
                            {
                                lines[TryInto::<usize>::try_into(ei).unwrap()]
                                    [TryInto::<usize>::try_into(begin).unwrap()] =
                                    Cell::Num { n: 0 };
                                begin += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    ans
}

fn part2(input: &str) -> u32 {
    let lines = input
        .split('\n')
        .map(|piece| {
            let mut ans = vec![];

            let mut n = 0;
            let mut count = 0;
            for c in piece.chars() {
                if c.is_ascii_digit() {
                    n *= 10;
                    n += c.to_digit(10).unwrap();
                    count += 1;
                } else {
                    for _ in 0..count {
                        ans.push(Cell::Num { n });
                    }
                    count = 0;
                    n = 0;
                    if c == '.' {
                        ans.push(Cell::Period);
                    } else {
                        ans.push(Cell::Char { c });
                    }
                }
            }
            for _ in 0..count {
                ans.push(Cell::Num { n });
            }
            ans
        })
        .collect::<Vec<_>>();
    let n = lines.len();
    let m = lines[0].len();

    let mut ans = 0;

    for i in 0i32..(n.try_into().unwrap()) {
        for j in 0i32..(m.try_into().unwrap()) {
            if let Cell::Char { c: _ } = lines[TryInto::<usize>::try_into(i).unwrap()]
                [TryInto::<usize>::try_into(j).unwrap()]
            {
                let mut neighboring_nums = vec![];
                let mut neighboring_locs = vec![];
                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        if ((di == 0) && (dj == 0))
                            || (i + di >= n.try_into().unwrap())
                            || (i + di < 0)
                            || (j + dj >= m.try_into().unwrap())
                            || (j + dj < 0)
                        {
                            continue;
                        }

                        let ei = i + di;
                        let ej = j + dj;
                        let _begin = ej;
                        if let Cell::Num { n } = lines[TryInto::<usize>::try_into(ei).unwrap()]
                            [TryInto::<usize>::try_into(ej).unwrap()]
                        {
                            neighboring_nums.push(n);
                            neighboring_locs.push((di, dj));
                        }
                    }
                }
                let mut count = 0;
                let mut product = 1;
                for (n, (i, j)) in neighboring_nums
                    .into_iter()
                    .zip(neighboring_locs.clone().into_iter())
                {
                    if (j == -1) || (i == 0) {
                        count += 1;
                        product *= n;
                    } else if j >= 0 {
                        let prev = (i, j - 1);
                        if !neighboring_locs.contains(&prev) {
                            count += 1;
                            product *= n;
                        }
                    }
                }
                if count == 2 {
                    ans += product;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(include_str!("../inputs/day-03/input_1.txt")) == 4361);
        assert!(part1(include_str!("../inputs/day-03/input_2.txt")) == 531561);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("../inputs/day-03/input_1.txt")) == 467835);
        assert!(part2(include_str!("../inputs/day-03/input_2.txt")) == 83279367);
    }
}
