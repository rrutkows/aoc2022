use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

type Coords = (usize, usize);

struct Visited {
    route_len: usize,
    exits: i32,
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

fn solve<F1, F2>(area: &Vec<Vec<u8>>, start: Coords, is_allowed: F1, is_end: F2) -> usize
where
    F1: Fn(&Coords, &Coords) -> bool,
    F2: Fn(&Coords) -> bool,
{
    let mut route_len = usize::MAX;
    let mut route = vec![start];
    let mut visited: HashMap<Coords, Visited> = HashMap::new();
    visited.insert(
        start,
        Visited {
            route_len: 0,
            exits: 0,
        },
    );

    while !route.is_empty() {
        let here = *route.last().unwrap();
        let (x, y) = here;

        let v = visited.get_mut(&here).unwrap();
        v.exits += 1;

        let next = match v.exits {
            1 if x + 1 < area[y].len() => (x + 1, y),
            2 if x > 0 => (x - 1, y),
            3 if y + 1 < area.len() => (x, y + 1),
            4 if y > 0 => (x, y - 1),
            5.. => {
                route.pop().unwrap();
                continue;
            }
            _ => continue,
        };

        if is_allowed(&here, &next)
            && route.len()
                < visited
                    .get(&next)
                    .map(|v| v.route_len)
                    .unwrap_or(usize::MAX)
        {
            visited.insert(
                next,
                Visited {
                    route_len: route.len(),
                    exits: 0,
                },
            );
            if is_end(&next) {
                route_len = std::cmp::min(route_len, route.len());
            } else {
                route.push(next);
            }
        }
    }

    route_len
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
