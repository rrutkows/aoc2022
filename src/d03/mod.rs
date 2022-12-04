use std::collections::HashSet;

mod input;

pub fn run01() -> i32 {
    input::DATA
        .lines()
        .map(|line| {
            let first_container_iter = line.chars().take(line.len() / 2);
            let first_container: HashSet<char> = HashSet::from_iter(first_container_iter);
            let wrong_type = line
                .chars()
                .skip(line.len() / 2)
                .find(|c| first_container.contains(c))
                .unwrap();
            get_priority(wrong_type)
        })
        .sum()
}

const GROUP_SIZE: i32 = 3;

pub fn run02() -> i32 {
    let mut lines = input::DATA.lines();
    let mut sum = 0;
    while let Some(first_in_group) = lines.next() {
        let mut common_items: HashSet<char> = HashSet::from_iter(first_in_group.chars());
        for _ in 2..=GROUP_SIZE {
            let common_items_iter = lines
                .next()
                .unwrap()
                .chars()
                .filter(|c| common_items.contains(c));
            common_items = HashSet::from_iter(common_items_iter);
        }

        let badge_type = common_items.into_iter().next().unwrap();
        sum += get_priority(badge_type);
    }

    sum
}

pub fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => 1 + (c as u8 - b'a') as i32,
        'A'..='Z' => 27 + (c as u8 - b'A') as i32,
        _ => 0,
    }
}
