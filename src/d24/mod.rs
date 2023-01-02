use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("input.txt");

type Coords = (usize, usize);

const R: usize = 0;
const D: usize = 1;
const L: usize = 2;
const U: usize = 3;

struct Map {
    blizzards: [Vec<u128>; 4],
    width: usize,
    height: usize,
}

impl Map {
    fn is_there_blizzard(&self, &(x, y): &Coords, t: usize) -> bool {
        let [r, d, l, u] = &self.blizzards;
        let w = self.width;
        r[y] & 1 << ((x + w - (t % w)) % w) != 0
            || d[(y + d.len() - (t % d.len())) % d.len()] & 1 << x != 0
            || l[y] & 1 << ((x + (t % w)) % w) != 0
            || u[(y + (t % u.len())) % u.len()] & 1 << x != 0
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    c: Coords,
    t: usize,
    min_t_possible: usize, // t + Manhattan distance to the destination
}

impl Node {
    fn new(c: Coords, t: usize, dest: &Coords) -> Self {
        Self {
            c,
            t,
            min_t_possible: t + dest.0.abs_diff(c.0) + dest.1.abs_diff(c.1),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_t_possible.cmp(&other.min_t_possible).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve01() -> usize {
    let map = parse();
    solve((0, 0), (map.width - 1, map.height - 1), 0, &map)
}

pub fn solve02() -> usize {
    let map = parse();
    [
        ((0, 0), (map.width - 1, map.height - 1)),
        ((map.width - 1, map.height - 1), (0, 0)),
        ((0, 0), (map.width - 1, map.height - 1)),
    ]
    .into_iter()
    .fold(0, |acc, (from, to)| solve(from, to, acc, &map))
}

fn solve(from: Coords, to: Coords, start_at: usize, map: &Map) -> usize {
    let mut result = usize::MAX;
    let mut q: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut starting_t = start_at;
    while result == usize::MAX {
        starting_t = (starting_t + 1..)
            .find(|t| !map.is_there_blizzard(&from, *t))
            .unwrap();
        q.push(Node::new(from, starting_t, &to));
        visited.insert((0, 0, starting_t));
        while let Some(n) = q.pop() {
            if n.c == to {
                result = result.min(n.t);
                continue;
            }

            if n.min_t_possible >= result {
                // This route cannot possibly be faster than one already found. Cut off branch.
                continue;
            }

            let mut nexts: Vec<Coords> = Vec::new();
            // Try waiting
            nexts.push(n.c);

            // Try moving
            let (x, y) = n.c;
            if x < map.width - 1 {
                nexts.push((x + 1, y));
            }
            if y < map.height - 1 {
                nexts.push((x, y + 1));
            }
            if x > 0 {
                nexts.push((x - 1, y));
            }
            if y > 0 {
                nexts.push((x, y - 1));
            }

            for next in nexts.into_iter() {
                if !visited.contains(&(next.0, next.1, n.t + 1))
                    && !map.is_there_blizzard(&next, n.t + 1)
                {
                    q.push(Node::new(next, n.t + 1, &to));
                    visited.insert((next.0, next.1, n.t + 1));
                }
            }
        }
    }

    result + 1
}

fn parse() -> Map {
    let mut lines = INPUT.lines();
    let mut blizzards: [Vec<u128>; 4] = std::array::from_fn(|_| Vec::new());
    let width = lines.next().unwrap().len() - 2;
    let mut y = 0;
    for line in lines {
        if line.starts_with("##") {
            break;
        }

        for v in blizzards.iter_mut() {
            v.push(0);
        }

        for (x, c) in line.chars().skip(1).enumerate() {
            let i = match c {
                '>' => R,
                'v' => D,
                '<' => L,
                '^' => U,
                '#' => break,
                _ => continue,
            };
            blizzards[i][y] |= 1 << x;
        }

        y += 1;
    }

    Map {
        blizzards,
        width,
        height: y,
    }
}
