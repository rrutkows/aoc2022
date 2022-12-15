use std::{collections::HashSet, ops::RangeInclusive};

const INPUT: &str = include_str!("input.txt");

const ROW: i32 = 2000000;

const X_RANGE: RangeInclusive<i32> = 0..=4000000;
const Y_RANGE: RangeInclusive<i32> = 0..=4000000;

type Coords = (i32, i32);

pub fn solve01() -> usize {
    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;
    let mut beacons_at_row: HashSet<i32> = HashSet::new();
    let ranges: Vec<RangeInclusive<i32>> = parse()
        .map(|((sx, sy), (bx, by))| {
            if by == ROW {
                beacons_at_row.insert(bx);
            }
            let d = (sx - bx).abs() + (sy - by).abs();
            let vertical_d = (sy - ROW).abs();
            let range = sx - d + vertical_d..=sx + d - vertical_d;
            if !range.is_empty() {
                minx = minx.min(*range.start());
                maxx = maxx.max(*range.end());
            }
            range
        })
        .collect();

    (minx..=maxx)
        .filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count()
        - beacons_at_row.len()
}

pub fn solve02() -> i64 {
    let sensors: Vec<(Coords, i32)> = parse()
        .map(|((sx, sy), (bx, by))| ((sx, sy), (sx - bx).abs() + (sy - by).abs()))
        .collect();
    for y in Y_RANGE {
        let mut x = *X_RANGE.start();
        while x <= *X_RANGE.end() {
            match sensors
                .iter()
                .find(|&&((sx, sy), d)| (sx - x).abs() + (sy - y).abs() <= d)
            {
                //move outside the range of sensor
                Some(((sx, sy), d)) => x = sx + d - (sy - y).abs() + 1,
                None => return x as i64 * 4000000 + y as i64,
            }
        }
    }

    -1
}

fn parse() -> impl Iterator<Item = (Coords, Coords)> {
    INPUT.lines().map(|line| {
        let numbers = String::from_iter(
            line.chars()
                .filter(|&c| c == ',' || c == '-' || c == ':' || c.is_ascii_digit()),
        );

        let (c1, c2) = numbers.split_once(':').unwrap();
        (
            c1.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap(),
            c2.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap(),
        )
    })
}
