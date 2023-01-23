use std::str::FromStr;

mod input;

type Stacks = Vec<Vec<char>>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn run(is_reordering_crane: bool) -> String {
    let mut stacks = parse_stacks();

    for m in input::MOVES.lines().map(parse_move) {
        // split_at_mut to have mutable references
        // to different items of the same Vec at the same time.
        let (s1, s2) = stacks.split_at_mut(usize::max(m.from, m.to));
        let (from, to) = if m.from > m.to {
            (&mut s2[0], &mut s1[m.to])
        } else {
            (&mut s1[m.from], &mut s2[0])
        };
        let from_len = from.len();
        let removed = from.drain(from_len - m.count..from_len);
        if is_reordering_crane {
            to.extend(removed.rev());
        } else {
            to.extend(removed);
        }
    }

    String::from_iter(
        stacks
            .iter()
            .filter(|stack| !stack.is_empty())
            .map(|stack| stack[stack.len() - 1]),
    )
}

fn parse_stacks() -> Stacks {
    let mut stacks: Stacks = Vec::new();
    for line in input::STACKS.lines().rev() {
        for (i, c) in line.chars().enumerate() {
            if i % 4 == 1 && !c.is_whitespace() {
                let stack_index = (i - 1) / 4;
                while stacks.len() <= stack_index {
                    stacks.push(Vec::new());
                }

                stacks[stack_index].push(c);
            }
        }
    }

    stacks
}

fn parse_move(s: &str) -> Move {
    let elements: Vec<&str> = s.split_whitespace().collect();
    Move {
        count: usize::from_str(elements[1]).unwrap(),
        from: usize::from_str(elements[3]).unwrap() - 1,
        to: usize::from_str(elements[5]).unwrap() - 1,
    }
}
