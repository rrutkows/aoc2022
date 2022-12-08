const INPUT: &str = include_str!("input.txt");

pub fn solve01() -> usize {
    let trees = parse();
    trees
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, tree)| {
                    (0..x).all(|i| row[i] < *tree)
                        || (x + 1..row.len()).all(|i| row[i] < *tree)
                        || (0..y).all(|i| trees[i][x] < *tree)
                        || (y + 1..trees.len()).all(|i| trees[i][x] < *tree)
                })
                .count()
        })
        .sum()
}

pub fn solve02() -> usize {
    let add_one = |x: usize| x + 1;
    let trees = parse();
    trees
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tree)| {
                    (0..x)
                        .rev()
                        .position(|i| row[i] >= *tree)
                        .map(add_one)
                        .unwrap_or(x)
                        * (x + 1..row.len())
                            .position(|i| row[i] >= *tree)
                            .map(add_one)
                            .unwrap_or(row.len() - x - 1)
                        * (0..y)
                            .rev()
                            .position(|i| trees[i][x] >= *tree)
                            .map(add_one)
                            .unwrap_or(y)
                        * (y + 1..trees.len())
                            .position(|i| trees[i][x] >= *tree)
                            .map(add_one)
                            .unwrap_or(trees.len() - y - 1)
                })
                .collect::<Vec<usize>>()
        })
        .max()
        .unwrap()
}

fn parse() -> Vec<Vec<u8>> {
    INPUT
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}
