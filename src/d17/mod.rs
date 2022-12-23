use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

const WIDTH: usize = 7;

struct Rock {
    width: usize,
    shape: &'static [u8],
}

const ROCKS: [Rock; 5] = [
    Rock {
        width: 4,
        shape: &[0b1111],
    },
    Rock {
        width: 3,
        shape: &[0b010, 0b111, 0b010],
    },
    Rock {
        width: 3,
        shape: &[0b111, 0b001, 0b001],
    },
    Rock {
        width: 1,
        shape: &[0b1, 0b1, 0b1, 0b1],
    },
    Rock {
        width: 2,
        shape: &[0b11, 0b11],
    },
];

enum Movement {
    Left,
    Right,
}

struct Snapshot {
    chamber_top: Vec<u8>,
    height: usize,
    rock_count: usize,
}

pub fn solve(rock_count: usize) -> usize {
    use Movement::*;
    let movements = parse();
    let mut i: usize = 0;
    let mut air_index: usize = 0;
    let mut chamber: Vec<u8> = Vec::new();
    let mut height: usize = 0;
    let mut history: HashMap<(usize, usize), Snapshot> = HashMap::new();

    while i < rock_count {
        let rock_index = i % ROCKS.len();
        let rock = &ROCKS[rock_index];
        let mut x = 2;
        let mut y = chamber.len() + 3;
        loop {
            match &movements[air_index] {
                Left => {
                    if x > 0
                        && rock.shape.iter().enumerate().all(|(r, shape)| {
                            y + r >= chamber.len()
                                || shape << (WIDTH - x - rock.width + 1) & chamber[y + r] == 0
                        })
                    {
                        x -= 1;
                    }
                }
                Right => {
                    if x + rock.width < WIDTH
                        && rock.shape.iter().enumerate().all(|(r, shape)| {
                            y + r >= chamber.len()
                                || shape << (WIDTH - x - rock.width - 1) & chamber[y + r] == 0
                        })
                    {
                        x += 1;
                    }
                }
            }

            air_index = (air_index + 1) % movements.len();

            if y == 0
                || rock.shape.iter().enumerate().any(|(r, shape)| {
                    y + r <= chamber.len()
                        && shape << (WIDTH - x - rock.width) & chamber[y + r - 1] != 0
                })
            {
                for (r, shape) in rock
                    .shape
                    .iter()
                    .enumerate()
                    .map(|(r, shape)| (r, shape << (WIDTH - x - rock.width)))
                {
                    if y + r < chamber.len() {
                        chamber[y + r] |= shape;
                    } else {
                        chamber.push(shape);
                    }
                }

                break;
            }

            y -= 1;
        }

        // Store the top of the chamber and splice the rest
        let mut sum: u8 = 0;
        let top: Vec<u8> = chamber
            .iter()
            .rev()
            .take_while(|row| {
                let result = sum != 0b1111111;
                sum |= **row;
                result
            })
            .copied()
            .collect();
        height += chamber.len() - top.len();
        chamber.splice(0..chamber.len() - top.len(), std::iter::empty());

        let history_key = (rock_index, air_index);
        if let Some(prev) = history.get(&history_key) {
            if top == *prev.chamber_top {
                let cycle_count = (rock_count - i - 1) / (i - prev.rock_count);
                i += cycle_count * (i - prev.rock_count);
                height += cycle_count * (chamber.len() + height - prev.height);
                history.clear();
            }
        }
        history.insert(
            history_key,
            Snapshot {
                chamber_top: top,
                height: chamber.len() + height,
                rock_count: i,
            },
        );

        i += 1;
    }

    chamber.len() + height
}

fn parse() -> Vec<Movement> {
    INPUT
        .chars()
        .map(|c| match c {
            '>' => Movement::Right,
            '<' => Movement::Left,
            _ => panic!(),
        })
        .collect()
}
