// use itertools::Itertools;

use std::collections::HashSet;

fn main() {
    // println!("{}", part1(include_str!("../inputs/day-10/input_2.txt")));
    println!(
        "{}",
        part2(include_str!("../inputs/day-10/part2/real.txt"), 26, 109)
    );
}

fn part1(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let m: usize = lines.len().try_into().unwrap();
    let mm: isize = lines.len().try_into().unwrap();
    let n: isize = lines[0].len().try_into().unwrap();

    let mut start_i = None;
    let mut start_j = None;

    let mut neighbors: Vec<Vec<Option<Vec<(isize, isize)>>>> = vec![];
    for ii in 0..m {
        let i = ii.try_into().unwrap();
        let mut new_vec: Vec<Option<Vec<(isize, isize)>>> = vec![];
        for (jj, c) in lines[ii].chars().enumerate() {
            let j = jj.try_into().unwrap();
            if c == '|' {
                new_vec.push(Some(vec![(i + 1, j), (i - 1, j)]));
            } else if c == '-' {
                new_vec.push(Some(vec![(i, j - 1), (i, j + 1)]));
            } else if c == 'L' {
                new_vec.push(Some(vec![(i, j + 1), (i - 1, j)]));
            } else if c == 'J' {
                new_vec.push(Some(vec![(i, j - 1), (i - 1, j)]));
            } else if c == '7' {
                new_vec.push(Some(vec![(i, j - 1), (i + 1, j)]));
            } else if c == 'F' {
                new_vec.push(Some(vec![(i, j + 1), (i + 1, j)]));
            } else if c == '.' {
                new_vec.push(Some(vec![]));
            } else if c == 'S' {
                new_vec.push(Some(vec![(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)]));
                start_i = Some(i);
                start_j = Some(j);
            }
        }
        neighbors.push(new_vec);
    }

    let mut curr_i = start_i;
    let mut curr_j = start_j;
    let mut prev_i = None;
    let mut prev_j = None;
    let mut loop_len = 0;
    while loop_len == 0 || curr_i != start_i || curr_j != start_j {
        let ii: usize = curr_i.unwrap().try_into().unwrap();
        let jj: usize = curr_j.unwrap().try_into().unwrap();
        let d = (curr_i, curr_j);
        (curr_i, curr_j) = neighbors[ii][jj]
            .clone()
            .unwrap()
            .iter()
            .filter_map(|(a, b)| {
                let aa: usize = (*a).try_into().unwrap();
                let bb: usize = (*b).try_into().unwrap();
                if (Some(*a) != prev_i || Some(*b) != prev_j)
                    && (0 <= *a && *a < mm && 0 <= *b && *b < n)
                    && (!neighbors[aa][bb].clone().unwrap().is_empty())
                {
                    Some((Some(*a), Some(*b)))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        loop_len += 1;
        (prev_i, prev_j) = d;
    }

    loop_len / 2
}

fn search(input: &str, i_start: usize, j_start: usize) -> HashSet<(usize, usize)> {
    let lines = input.split('\n').collect::<Vec<_>>();
    let m: usize = lines.len().try_into().unwrap();
    let mm: isize = lines.len().try_into().unwrap();
    let n: isize = lines[0].len().try_into().unwrap();

    let mut start_i = None;
    let mut start_j = None;

    let mut neighbors: Vec<Vec<Option<Vec<(isize, isize)>>>> = vec![];
    let mut pipes = HashSet::new();
    for ii in 0..m {
        let i = ii.try_into().unwrap();
        let mut new_vec: Vec<Option<Vec<(isize, isize)>>> = vec![];
        for (jj, c) in lines[ii].chars().enumerate() {
            let j = jj.try_into().unwrap();
            if c == '|' {
                new_vec.push(Some(vec![(i + 1, j), (i - 1, j)]));
            } else if c == '-' {
                new_vec.push(Some(vec![(i, j - 1), (i, j + 1)]));
            } else if c == 'L' {
                new_vec.push(Some(vec![(i, j + 1), (i - 1, j)]));
            } else if c == 'J' {
                new_vec.push(Some(vec![(i, j - 1), (i - 1, j)]));
            } else if c == '7' {
                new_vec.push(Some(vec![(i, j - 1), (i + 1, j)]));
            } else if c == 'F' {
                new_vec.push(Some(vec![(i, j + 1), (i + 1, j)]));
            } else {
                new_vec.push(Some(vec![]));
            }
            if (ii == i_start) && (jj == j_start) {
                start_i = Some(i);
                start_j = Some(j);
                pipes.insert((ii, jj));
            }
        }
        neighbors.push(new_vec);
    }

    let mut curr_i = start_i;
    let mut curr_j = start_j;
    let mut prev_i = None;
    let mut prev_j = None;
    let mut loop_len = 0;
    while loop_len == 0 || curr_i != start_i || curr_j != start_j {
        let ii: usize = curr_i.unwrap().try_into().unwrap();
        let jj: usize = curr_j.unwrap().try_into().unwrap();
        let d = (curr_i, curr_j);
        (curr_i, curr_j) = neighbors[ii][jj]
            .clone()
            .unwrap()
            .iter()
            .filter_map(|(a, b)| {
                let aa: usize = (*a).try_into().unwrap();
                let bb: usize = (*b).try_into().unwrap();
                if (Some(*a) != prev_i || Some(*b) != prev_j)
                    && (0 <= *a && *a < mm && 0 <= *b && *b < n)
                    && (!neighbors[aa][bb].clone().unwrap().is_empty())
                {
                    pipes.insert((aa, bb));
                    Some((Some(*a), Some(*b)))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        loop_len += 1;
        (prev_i, prev_j) = d;
    }
    assert!(loop_len == pipes.len());
    pipes
}

enum Pipe {
    UpDown,
    LeftRight,
    L,
    J,
    F,
    Seven,
    No,
}

fn part2(input: &str, i: usize, j: usize) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let m: usize = lines.len().try_into().unwrap();
    let n: usize = lines[0].len().try_into().unwrap();

    let useful_pipes = search(input, i, j);

    let mut neighbors: Vec<Vec<_>> = vec![];
    for ii in 0..m {
        let mut new_vec: Vec<_> = vec![];
        for (jj, c) in lines[ii].chars().enumerate() {
            if !useful_pipes.contains(&(ii, jj)) {
                new_vec.push(Pipe::No);
            } else if c == '|' {
                new_vec.push(Pipe::UpDown);
            } else if c == '-' {
                new_vec.push(Pipe::LeftRight);
            } else if c == 'L' {
                new_vec.push(Pipe::L);
            } else if c == 'J' {
                new_vec.push(Pipe::J);
            } else if c == '7' {
                new_vec.push(Pipe::Seven);
            } else if c == 'F' {
                new_vec.push(Pipe::F);
            }
        }
        neighbors.push(new_vec);
    }

    let mut trip_neighbors: Vec<Vec<Vec<(usize, usize)>>> = (0..(3 * m))
        .map(|_| (0..3 * n).map(|_| vec![]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..m {
        for j in 0..n {
            match neighbors[i][j] {
                Pipe::UpDown => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * i, 3 * j + 1), (3 * i + 2, 3 * j + 1)];
                    trip_neighbors[3 * i][3 * j + 1] = vec![(3 * i + 1, 3 * j + 1)];
                    if i > 0 {
                        trip_neighbors[3 * i][3 * j + 1].push((3 * i - 1, 3 * j + 1));
                    }
                    trip_neighbors[3 * i + 2][3 * j + 1] =
                        vec![(3 * i + 1, 3 * j + 1), (3 * i + 3, 3 * j + 1)];
                }
                Pipe::LeftRight => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * j + 1, 3 * j), (3 * i + 1, 3 * j + 2)];
                    trip_neighbors[3 * i + 1][3 * j] = vec![(3 * i + 1, 3 * j + 1)];
                    if j > 0 {
                        trip_neighbors[3 * i + 1][3 * j].push((3 * i + 1, 3 * j - 1));
                    }
                    trip_neighbors[3 * i + 1][3 * j + 2] =
                        vec![(3 * i + 1, 3 * j + 1), (3 * i + 1, 3 * j + 3)];
                }
                Pipe::L => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * i, 3 * j + 1), (3 * i + 1, 3 * j + 2)];
                    trip_neighbors[3 * i][3 * j + 1] = vec![(3 * i + 1, 3 * j + 1)];
                    if i > 0 {
                        trip_neighbors[3 * i][3 * j + 1].push((3 * i - 1, 3 * j + 1));
                    }
                    trip_neighbors[3 * i + 1][3 * j + 2] =
                        vec![(3 * i + 1, 3 * j + 1), (3 * i + 1, 3 * j + 2)];
                }
                Pipe::J => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * i, 3 * j + 1), (3 * i + 1, 3 * j)];
                    trip_neighbors[3 * i][3 * j + 1] = vec![(3 * i + 1, 3 * j + 1)];
                    if i > 0 {
                        trip_neighbors[3 * i][3 * j + 1].push((3 * i - 1, 3 * j + 1));
                    }
                    trip_neighbors[3 * i + 1][3 * j] = vec![(3 * i + 1, 3 * j + 1)];
                    if j > 0 {
                        trip_neighbors[3 * i + 1][3 * j].push((3 * i + 1, 3 * j - 1));
                    }
                }
                Pipe::F => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * i + 2, 3 * j + 1), (3 * i + 1, 3 * j + 2)];
                    trip_neighbors[3 * i + 2][3 * j + 1] = vec![(3 * i + 1, 3 * j + 1)];
                    trip_neighbors[3 * i + 2][3 * j + 1].push((3 * i + 3, 3 * j + 1));
                    trip_neighbors[3 * i + 1][3 * j + 2] = vec![(3 * i + 1, 3 * j + 1)];
                    trip_neighbors[3 * i + 1][3 * j + 2].push((3 * i + 1, 3 * j + 3));
                }
                Pipe::Seven => {
                    trip_neighbors[3 * i + 1][3 * j + 1] =
                        vec![(3 * i + 1, 3 * j), (3 * i + 2, 3 * j + 1)];
                    trip_neighbors[3 * i + 1][3 * j] = vec![(3 * i + 1, 3 * j + 1)];
                    if j > 0 {
                        trip_neighbors[3 * i + 1][3 * j].push((3 * i + 1, 3 * j - 1));
                    }
                    trip_neighbors[3 * i + 2][3 * j + 1] =
                        vec![(3 * i + 1, 3 * j + 1), (3 * i + 2, 3 * j + 1)];
                }
                Pipe::No => {}
            }
        }
    }
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    dots.insert((0, 0));
    let mut last_size: usize = 0;
    while last_size < dots.len() {
        println!("{}", dots.len());
        last_size = dots.len();
        let mut next_dots: HashSet<(usize, usize)> = HashSet::new();
        for dot in dots.iter() {
            let x = dot.0;
            let y = dot.1;
            let mut dxs = vec![x];
            if x > 0 {
                dxs.push(x - 1);
            }
            if x < 3 * m - 1 {
                dxs.push(x + 1);
            }
            let mut dys = vec![y];
            if y > 0 {
                dys.push(y - 1);
            }
            if y < 3 * n - 1 {
                dys.push(y + 1);
            }
            for nx in dxs {
                for ny in dys.iter() {
                    if trip_neighbors[nx][*ny].is_empty() {
                        next_dots.insert((nx, *ny));
                    }
                }
            }
        }
        dots = next_dots;
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if (trip_neighbors[3 * i + 1][3 * j + 1].is_empty())
                && !dots.contains(&(3 * i + 1, 3 * j + 1))
            {
                println!("{} {}", i, j);
                ans += 1;
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
        assert!(part1(include_str!("../inputs/day-10/input_1.txt")) == 0);
    }

    #[test]
    fn test_part2() {}
}
