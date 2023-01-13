use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

type Coords = [u8; 3];

struct Map {
    content: [[u32; 32]; 32],
}

impl Map {
    fn new() -> Self {
        Self {
            content: [[0; 32]; 32],
        }
    }

    fn set(&mut self, c: &Coords) {
        self.content[c[0] as usize][c[1] as usize] |= 1 << c[2];
    }

    fn is_set(&self, c: &Coords) -> bool {
        self.content[c[0] as usize][c[1] as usize] & (1 << c[2]) != 0
    }
}

impl FromIterator<Coords> for Map {
    fn from_iter<T: IntoIterator<Item = Coords>>(iter: T) -> Self {
        iter.into_iter().fold(Self::new(), |mut acc, c| {
            acc.set(&c);
            acc
        })
    }
}

pub fn solve01() -> usize {
    let coords_list: Vec<Coords> = parse().collect();
    coords_list.len() * 6
        - 2 * coords_list
            .iter()
            .enumerate()
            .flat_map(|(i, c1)| {
                coords_list
                    .iter()
                    .skip(i + 1)
                    .filter(|c2| {
                        std::iter::zip(*c1, **c2)
                            .map(|(c1, c2)| c1.abs_diff(c2))
                            .sum::<u8>()
                            == 1
                    })
            })
            .count()
}

pub fn solve02() -> usize {
    let mut count: usize = 0;
    let mut max: [u8; 3] = [0; 3];
    let lava = Map::from_iter(parse().map(|coords| {
        for (i, c) in coords.iter().enumerate() {
            max[i] = max[i].max(*c);
        }
        coords.map(|c| c + 1)
    }));
    let mut visited = Map::new();
    visited.set(&[0, 0, 0]);
    let mut q: VecDeque<Coords> = VecDeque::new();
    q.push_back([0, 0, 0]);

    while let Some(c) = q.pop_front() {
        let mut nexts: Vec<Coords> = Vec::new();
        for i in 0..3 {
            if c[i] > 0 {
                nexts.push(std::array::from_fn(
                    |j| if i == j { c[j] - 1 } else { c[j] },
                ));
            }

            if c[i] < max[i] + 2 {
                nexts.push(std::array::from_fn(
                    |j| if i == j { c[j] + 1 } else { c[j] },
                ));
            }
        }

        for next in nexts.into_iter() {
            if lava.is_set(&next) {
                count += 1;
            } else if !visited.is_set(&next) {
                visited.set(&next);
                q.push_back(next);
            }
        }
    }

    count
}

fn parse() -> impl Iterator<Item = Coords> {
    INPUT.lines().map(|line| {
        let mut coords = line.split(',');
        std::array::from_fn(|_| coords.next().unwrap().parse().unwrap())
    })
}
