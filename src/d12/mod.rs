use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input.txt");

type Coords = (usize, usize);

struct Node {
    coords: Coords,
    route_len: usize,
}

pub fn solve01() -> usize {
    let (area, start, end) = parse();
    solve(
        &area,
        start,
        |&(x1, y1), &(x2, y2)| area[y2][x2] <= area[y1][x1] + 1,
        |&coords| coords == end,
    )
}

pub fn solve02() -> usize {
    let (area, _, end) = parse();
    solve(
        &area,
        end,
        |&(x1, y1), &(x2, y2)| area[y2][x2] >= area[y1][x1] - 1,
        |&coords| area[coords.1][coords.0] == 0,
    )
}

fn solve<F1, F2>(area: &[Vec<u8>], start: Coords, is_allowed: F1, is_end: F2) -> usize
where
    F1: Fn(&Coords, &Coords) -> bool,
    F2: Fn(&Coords) -> bool,
{
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashMap<Coords, usize> = HashMap::new();
    queue.push_back(Node {
        coords: start,
        route_len: 0,
    });
    while let Some(here) = queue.pop_front() {
        // Because all edges have the same cost,
        // nodes at the beginning of the queue are guaranteed to be the shortest route.
        if is_end(&here.coords) {
            return here.route_len;
        }

        for n in neighbours(here.coords, area) {
            if is_allowed(&here.coords, &n)
                && here.route_len + 1 < *visited.get(&n).unwrap_or(&usize::MAX)
            {
                visited.insert(n, here.route_len + 1);
                queue.push_back(Node {
                    coords: n,
                    route_len: here.route_len + 1,
                });
            }
        }
    }

    usize::MAX
}

fn neighbours(here: Coords, area: &[Vec<u8>]) -> impl Iterator<Item = Coords> {
    let (x, y) = here;
    let mut neighbours: Vec<Coords> = Vec::with_capacity(4);
    if x + 1 < area[y].len() {
        neighbours.push((x + 1, y));
    }
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y + 1 < area.len() {
        neighbours.push((x, y + 1));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    neighbours.into_iter()
}

fn parse() -> (Vec<Vec<u8>>, Coords, Coords) {
    let mut start: Coords = (0, 0);
    let mut end: Coords = (0, 0);
    let area: Vec<Vec<u8>> = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        0
                    }
                    'E' => {
                        end = (x, y);
                        b'z' - b'a'
                    }
                    _ => c as u8 - b'a',
                })
                .collect()
        })
        .collect();
    (area, start, end)
}
