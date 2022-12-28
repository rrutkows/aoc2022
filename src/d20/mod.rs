const INPUT: &str = include_str!("input.txt");

pub fn solve(descryption_key: isize, mixing_count: usize) -> isize {
    // Stores the current index of the value and the value.
    let mut v: Vec<(usize, isize)> = parse()
        .enumerate()
        .map(|(i, v)| (i, v * descryption_key))
        .collect();
    // Stores the order of the original indices.
    let mut idx: Vec<usize> = (0..v.len()).collect();

    let ilen = v.len() as isize;
    for _ in 0..mixing_count {
        for i in 0..v.len() {
            let (from, value) = v[i];
            let tmp = from as isize + (value % (ilen - 1));
            let to = if value < 0 && tmp <= 0 {
                (tmp - 1).rem_euclid(ilen)
            } else if value > 0 && tmp >= ilen - 1 {
                (tmp + 1).rem_euclid(ilen)
            } else {
                tmp.rem_euclid(ilen)
            } as usize;

            let moved_idx = idx[from];
            v[idx[from]] = (to, v[idx[from]].1);
            if to < from {
                for j in (to + 1..from + 1).rev() {
                    v[idx[j - 1]] = (j, v[idx[j - 1]].1);
                    idx[j] = idx[j - 1];
                }
            } else {
                for j in from..to {
                    v[idx[j + 1]] = (j, v[idx[j + 1]].1);
                    idx[j] = idx[j + 1];
                }
            }
            idx[to] = moved_idx;
        }
    }

    let zero_idx = v.iter().find(|(_, v)| *v == 0).unwrap().0;
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| v[idx[(zero_idx + i) % idx.len()]].1)
        .sum()
}

fn parse() -> impl Iterator<Item = isize> {
    INPUT.lines().map(|line| line.parse().unwrap())
}
