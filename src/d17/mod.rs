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

pub fn solve(rock_count: usize) -> usize {
    use Movement::*;
    let movements = parse();
    let mut air_index: usize = 0;
    let mut chamber: Vec<u8> = Vec::new();

    for i in 0..rock_count {
        let rock = &ROCKS[i % ROCKS.len()];
        let mut x = 2;
        let mut y = chamber.len() + 3;
        loop {
            match &movements[air_index % movements.len()] {
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

            air_index += 1;

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
    }

    chamber.len()
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
