use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

type Coords = (i32, i32);
type Map = HashMap<Coords, usize>;

const N: u8 = 0;
const S: u8 = 1;
const W: u8 = 2;
const E: u8 = 3;

pub fn solve01() -> i32 {
    let (_, empty_space) = solve(Some(10));
    empty_space
}

pub fn solve02() -> usize {
    let (rounds, _) = solve(None);
    rounds
}

fn solve(round_count: Option<usize>) -> (usize, i32) {
    let (mut map, mut elves) = parse();

    let mut r = 0;
    while round_count.map(|rc| rc > r).unwrap_or(true) {
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
                map.remove(&elves[i]);
                map.insert(k, i);
                elves[i] = k;
            });
        }

        r += 1;
    }

    let (min_x, min_y, max_x, max_y) = elves.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
        },
    );

    (
        r + 1,
        (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32,
    )
}

fn can_move(&(x, y): &Coords, d: u8, map: &Map) -> bool {
    let adjacent = match d {
        N => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
        S => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
        W => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
        E => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        _ => panic!("bad direction"),
    };
    adjacent.into_iter().all(|c| !map.contains_key(&c))
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

fn parse() -> (Map, Vec<Coords>) {
    let mut map: Map = HashMap::new();
    let mut elves: Vec<Coords> = Vec::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    let c = (x as i32, y as i32);
                    map.insert(c, elves.len());
                    elves.push(c);
                }
                _ => {}
            }
        }
    }

    (map, elves)
}
