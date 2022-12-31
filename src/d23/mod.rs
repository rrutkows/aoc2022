use std::{collections::HashMap, iter};

const INPUT: &str = include_str!("input.txt");

type Coords = (usize, usize);
type Map = Vec<Vec<Option<usize>>>;

const N: u8 = 0;
const S: u8 = 1;
const W: u8 = 2;
const E: u8 = 3;

pub fn solve01() -> usize {
    solve(10)
}

fn solve(round_count: usize) -> usize {
    let (mut map, mut elves) = parse(round_count + 1);

    for r in 0..round_count {
        // targets:
        // Some - exactly one elf proposes to move there,
        // None - more than one elf proposes to move there
        let mut targets: HashMap<Coords, Option<usize>> = HashMap::new();

        for (i, e) in elves.iter().enumerate() {
            let possible_directions: Vec<u8> = (r..r + 4)
                .map(|d| (d % 4) as u8)
                .filter(|d| can_move(e, *d, &map))
                .collect();
            // If an elf can go anywhere, ie len()==4, this means he has no neighbours. He doesn't move then.
            if (1..=3).contains(&possible_directions.len()) {
                let target = make_move(e, possible_directions[0]);
                if targets.contains_key(&target) {
                    targets.insert(target, None);
                } else {
                    targets.insert(target, Some(i));
                }
            }
        }

        if targets.is_empty() {
            break;
        }

        for (k, v) in targets.into_iter() {
            v.map(|i| {
                map[elves[i].1][elves[i].0] = None;
                map[k.1][k.0] = Some(i);
                elves[i] = k;
            });
        }
    }

    let (min_x, min_y, max_x, max_y) = elves.iter().fold(
        (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
        },
    );
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len()
}

fn can_move(&(x, y): &Coords, d: u8, map: &Map) -> bool {
    let adjacent = match d {
        N => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
        S => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
        W => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
        E => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        _ => panic!("bad direction"),
    };
    adjacent.into_iter().all(|(x, y)| map[y][x].is_none())
}

fn make_move(&(x, y): &Coords, d: u8) -> Coords {
    match d {
        N => (x, y - 1),
        S => (x, y + 1),
        W => (x - 1, y),
        E => (x + 1, y),
        _ => panic!("bad direction"),
    }
}

fn _get_adjacent(&(x, y): &Coords) -> [Coords; 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn _dump(map: &Map) {
    for row in map.iter() {
        println!(
            "{}",
            String::from_iter(row.iter().map(|t| t.map(|_| '#').unwrap_or('.')))
        )
    }
}

fn parse(buffer: usize) -> (Map, Vec<Coords>) {
    let mut lines = INPUT.lines().peekable();
    let width = lines.peek().unwrap().len() + 2 * buffer;
    let mut map: Map = Vec::from_iter(
        iter::repeat_with(|| Vec::from_iter(iter::repeat(None).take(width))).take(buffer),
    );
    let mut elves: Vec<Coords> = Vec::new();

    let mut y = buffer;
    for line in lines {
        let mut row: Vec<Option<usize>> = Vec::new();
        row.resize(buffer, None);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    row.push(Some(elves.len()));
                    elves.push((x + buffer, y));
                }
                _ => row.push(None),
            }
        }
        row.resize(width, None);
        map.push(row);
        y += 1;
    }

    map.extend(iter::repeat_with(|| Vec::from_iter(iter::repeat(None).take(width))).take(buffer));

    (map, elves)
}
