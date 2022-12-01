use std::iter;

mod input;

pub fn run(top_count: usize) -> i32 {
    let mut top: Vec<i32> = Vec::from_iter(iter::repeat(0).take(top_count));
    let mut current = 0;
    for input in input::DATA.lines() {
        if input.is_empty() {
            update_top(&mut top, current);
            current = 0;
        } else {
            current += input.parse::<i32>().unwrap();
        }
    }

    update_top(&mut top, current);
    top.into_iter().sum()
}

fn update_top(top: &mut [i32], input: i32) {
    let mut current = input;
    for top_item in top.iter_mut() {
        // Guarantess top will always be sorted from largest to lowest
        // and the previous lowest will be removed.
        if current > *top_item {
            (*top_item, current) = (current, *top_item);
        }
    }
}
