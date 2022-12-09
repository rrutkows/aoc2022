use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

pub fn solve(knot_count: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = Vec::from_iter(std::iter::repeat((0, 0)).take(knot_count));
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert((0, 0));
    for line in INPUT.lines() {
        let (direction, times) = line
            .split_once(' ')
            .map(|(l1, l2)| (l1, usize::from_str(l2).unwrap()))
            .unwrap();
        let head_move = match direction {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!("bad input"),
        };
        for _ in 0..times {
            knots[0] = (knots[0].0 + head_move.0, knots[0].1 + head_move.1);
            for j in 1..knot_count {
                let delta = (knots[j - 1].0 - knots[j].0, knots[j - 1].1 - knots[j].1);
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    knots[j] = (knots[j].0 + delta.0.signum(), knots[j].1 + delta.1.signum());
                    if j == knot_count - 1 {
                        tail_visited.insert(knots[j]);
                    }
                }
            }
        }
    }

    tail_visited.len()
}
