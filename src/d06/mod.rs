use std::collections::HashSet;

mod input;

pub fn solve(marker_size: usize) -> i32 {
    for i in marker_size - 1..input::DATA.len() {
        let h: HashSet<char> = HashSet::from_iter(input::DATA[i + 1 - marker_size..=i].chars());
        if h.len() == marker_size {
            return (i + 1) as i32;
        }
    }

    -1
}
