const INPUT: &str = include_str!("input.txt");

// 0 - ore
// 1 - clay
// 2 - obsidian
// 3 - geode
#[derive(Debug)]
struct Blueprint {
    id: u8,
    robots: [[u8; 4]; 4],
}

struct Snapshot {
    time_passed: u8,
    robots: [u32; 4],
    minerals: [u32; 4],
}

impl Snapshot {
    fn new() -> Self {
        Self {
            time_passed: 0,
            robots: [1, 0, 0, 0],
            minerals: [0; 4],
        }
    }

    fn next(&self, bprint: &Blueprint, new_robot: usize) -> Self {
        let time_needed = 1 + bprint.robots[new_robot]
            .iter()
            .enumerate()
            .map(|(i, cost)| {
                if *cost as u32 > self.minerals[i] {
                    (*cost as u32 - self.minerals[i] + self.robots[i] - 1) / self.robots[i]
                } else {
                    0
                }
            })
            .max()
            .unwrap();
        Self {
            time_passed: self.time_passed + time_needed as u8,
            robots: std::array::from_fn(|i| self.robots[i] + if i == new_robot { 1 } else { 0 }),
            minerals: std::array::from_fn(|i| {
                self.minerals[i] + self.robots[i] * time_needed - bprint.robots[new_robot][i] as u32
            }),
        }
    }
}

pub fn solve01() -> u32 {
    parse()
        .map(|bprint| bprint.id as u32 * use_bprint(&bprint, 24))
        .sum::<u32>()
}

pub fn solve02() -> u32 {
    parse()
        .take(3)
        .map(|bprint| use_bprint(&bprint, 32))
        .product()
}

fn use_bprint(bprint: &Blueprint, time: u8) -> u32 {
    let mut geodes: u32 = 0;
    let mut q: Vec<Snapshot> = Vec::new();
    q.push(Snapshot::new());
    while let Some(current) = q.pop() {
        let time_left = (time - current.time_passed) as u32;
        geodes = geodes.max(current.minerals[3] + time_left * current.robots[3]);
        let geodes_possible =
            current.minerals[3] + time_left * current.robots[3] + (1..time_left).sum::<u32>();
        if geodes_possible > geodes {
            for affordable_robot in bprint
                .robots
                .iter()
                .enumerate()
                .rev() //try building geode robots first
                .filter(|(_, cost)| {
                    time_left > 1
                        && cost.iter().enumerate().all(|(j, c)| {
                            *c as u32 <= current.minerals[j] + (time_left - 1) * current.robots[j]
                        })
                })
                .map(|(i, _)| i)
            {
                q.push(current.next(bprint, affordable_robot));
            }
        }
    }

    geodes
}

fn parse() -> impl Iterator<Item = Blueprint> {
    INPUT.lines().map(|line| {
        let (id_part, bprint_part) = line.split_once(": ").unwrap();
        let id: u8 = id_part
            .split_once(' ')
            .map(|(_, id)| id.parse().unwrap())
            .unwrap();
        let bprint: Vec<&str> = bprint_part.split(' ').collect();
        Blueprint {
            id,
            robots: [
                [bprint[4].parse().unwrap(), 0, 0, 0],
                [bprint[10].parse().unwrap(), 0, 0, 0],
                [
                    bprint[16].parse().unwrap(),
                    bprint[19].parse().unwrap(),
                    0,
                    0,
                ],
                [
                    bprint[25].parse().unwrap(),
                    0,
                    bprint[28].parse().unwrap(),
                    0,
                ],
            ],
        }
    })
}
