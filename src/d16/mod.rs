use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input.txt");

const TIME: usize = 30;

const TIME_TO_LEARN: usize = 4;

#[derive(Debug)]
struct Valve {
    _id: String,
    flow_rate: i32,
    tunnels: Vec<usize>,
}

#[derive(Debug)]
struct PlanElement {
    destination: usize,
    time: usize,
    pressure: i32,
    opened: u64,
    team_member: usize,
}

pub fn solve(team_size: usize) -> i32 {
    let (valves, start) = parse();

    let mut max = 0;

    let time_left = TIME - TIME_TO_LEARN * (team_size - 1);

    let openable: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(i, _)| i)
        .collect();

    let mut distance_cache: HashMap<u64, usize> = HashMap::new();

    let mut plan: Vec<PlanElement> = vec![PlanElement {
        destination: start,
        time: 0,
        pressure: 0,
        opened: 0,
        team_member: 1,
    }];

    while let Some(el) = plan.pop() {
        max = max.max(el.pressure);
        if el.time + 2 >= time_left {
            continue;
        }

        plan.extend(
            openable
                .iter()
                .filter(|i| !is_set(**i, el.opened))
                .map(|&i| {
                    (
                        i,
                        find_path(
                            el.destination,
                            i,
                            &valves,
                            time_left - el.time - 2,
                            &mut distance_cache,
                        ),
                    )
                })
                .filter(|(_, t)| t.is_some())
                .map(|(i, t)| PlanElement {
                    destination: i,
                    time: el.time + t.unwrap() + 1,
                    pressure: el.pressure
                        + valves[i].flow_rate * (time_left - el.time - t.unwrap() - 1) as i32,
                    opened: set(i, el.opened),
                    team_member: el.team_member,
                }),
        );

        // What if the other team members open the rest of the valves?
        if el.team_member < team_size {
            plan.push(PlanElement {
                destination: start,
                time: 0,
                pressure: el.pressure,
                opened: el.opened,
                team_member: el.team_member + 1,
            });
        }
    }

    max
}

fn find_path(
    from: usize,
    to: usize,
    valves: &[Valve],
    time_left: usize,
    distance_cache: &mut HashMap<u64, usize>,
) -> Option<usize> {
    let key = (1u64 << from) + (1u64 << to);
    if let Some(distance) = distance_cache.get(&key) {
        return (*distance <= time_left).then_some(*distance);
    }

    let mut visited = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((from, 0));
    while let Some((current, time)) = q.pop_front() {
        visited = set(current, visited);

        if current == to {
            distance_cache.insert(key, time);
            return (time <= time_left).then_some(time);
        }

        q.extend(
            valves[current]
                .tunnels
                .iter()
                .filter(|&&t| !is_set(t, visited))
                .map(|&t| (t, time + 1)),
        );
    }

    None
}

fn set(index: usize, content: u64) -> u64 {
    content | (1 << index)
}

fn is_set(index: usize, content: u64) -> bool {
    content & (1 << index) != 0
}

fn parse() -> (Vec<Valve>, usize) {
    let mut id_to_number: HashMap<&str, usize> = HashMap::new();
    let mut start: usize = 0;
    let mut valves: Vec<Valve> = INPUT
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let fragments: Vec<&str> = line.splitn(7, &[' ', '=', ';']).collect();
            let id = fragments[1];
            if id == "AA" {
                start = i;
            }
            id_to_number.insert(id, i);
            Valve {
                _id: fragments[1].to_owned(),
                flow_rate: fragments[5].parse().unwrap(),
                tunnels: Vec::new(),
            }
        })
        .collect();

    for (i, line) in INPUT.lines().enumerate() {
        let fragments: Vec<&str> = line.splitn(10, ' ').collect();
        valves[i].tunnels.extend(
            fragments[9]
                .split(", ")
                .map(|s| id_to_number.get(s).unwrap()),
        );
    }
    (valves, start)
}
