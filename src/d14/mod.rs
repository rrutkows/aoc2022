use std::{cmp, collections::HashSet};

const INPUT: &str = include_str!("input.txt");

type Coords = (usize, usize);

pub fn solve01() -> usize {
    solve(false)
}

pub fn solve02() -> usize {
    solve(true)
}

fn solve(has_floor: bool) -> usize {
    let (mut blocked, bottom) = parse();
    let stone_count = blocked.len();
    let mut q: Vec<Coords> = Vec::new();
    q.push((500, 0));
    while let Some(&(x, y)) = q.last() {
        let mut found_space = false;
        if y <= bottom {
            for next in [(x + 1, y + 1), (x - 1, y + 1), (x, y + 1)]
                .into_iter()
                .filter(|next| !blocked.contains(next))
            {
                q.push(next);
                found_space = true;
            }
        }
        if !found_space {
            blocked.insert((x, y));
            q.pop();
        } else if !has_floor && y >= bottom {
            break;
        }
    }

    blocked.len() - stone_count
}

fn parse() -> (HashSet<Coords>, usize) {
    let mut blocked: HashSet<Coords> = HashSet::new();
    let mut bottom: usize = 0;
    for line in INPUT.lines() {
        line.split(" -> ")
            .fold(None, |acc: Option<Coords>, coords_str| {
                let (x, y): Coords = coords_str
                    .split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap();
                bottom = cmp::max(bottom, y);
                if let Some(last) = acc {
                    if x == last.0 {
                        for i in cmp::min(y, last.1)..=cmp::max(y, last.1) {
                            blocked.insert((x, i));
                        }
                    } else if y == last.1 {
                        for i in cmp::min(x, last.0)..=cmp::max(x, last.0) {
                            blocked.insert((i, y));
                        }
                    }
                } else {
                    blocked.insert((x, y));
                }
                Some((x, y))
            });
    }

    (blocked, bottom)
}
