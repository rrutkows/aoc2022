use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const FIRST: i32 = 20;
const INCREMENT: i32 = 40;
const LAST: i32 = 220;

const WIDTH: i32 = 40;

pub fn solve() -> i32 {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 0;

    let mut tick = |x| {
        if (x - 1..=x + 1).contains(&(cycle % WIDTH)) {
            print!("#");
        } else {
            print!(".");
        }
        cycle += 1;
        if cycle % WIDTH == 0 {
            println!();
        }
        if (cycle - FIRST) % INCREMENT == 0 && cycle <= LAST {
            sum += cycle * x;
        }
    };

    for line in INPUT.lines() {
        tick(x);

        if let Some((_, argument)) = line.split_once(' ') {
            tick(x);

            x += i32::from_str(argument).unwrap();
        }
    }

    sum
}
