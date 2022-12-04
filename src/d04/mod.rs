use std::str::FromStr;

mod input;

type PairAssignment = [[i32; 2]; 2];

pub fn run01() -> i32 {
    run(|a| fully_contains(&a[0], &a[1]) || fully_contains(&a[1], &a[0]))
}

pub fn run02() -> i32 {
    run(|a| overlaps(&a[0], &a[1]))
}

fn run<F>(condition: F) -> i32
where
    F: Fn(&PairAssignment) -> bool,
{
    input::DATA.lines().map(parse).filter(condition).count() as i32
}

fn fully_contains<T: PartialOrd>(container: &[T; 2], contained: &[T; 2]) -> bool {
    container[0].le(&contained[0]) && container[1].ge(&contained[1])
}

fn overlaps<T: PartialOrd>(a1: &[T; 2], a2: &[T; 2]) -> bool {
    a1[0].le(&a2[1]) && a1[1].ge(&a2[0])
}

fn parse(line: &str) -> PairAssignment {
    let mut pair = line.split(',');
    std::array::from_fn(|_| {
        let mut assignment = pair.next().unwrap().split('-');
        [
            i32::from_str(assignment.next().unwrap()).unwrap(),
            i32::from_str(assignment.next().unwrap()).unwrap(),
        ]
    })
}
