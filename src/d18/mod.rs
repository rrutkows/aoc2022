const INPUT: &str = include_str!("input.txt");

type Coords = [u8; 3];

pub fn solve() -> usize {
    let coords_list: Vec<Coords> = parse().collect();
    coords_list.len() * 6
        - 2 * coords_list
            .iter()
            .enumerate()
            .map(|(i, c1)| {
                coords_list
                    .iter()
                    .skip(i + 1)
                    .filter(|c2| {
                        std::iter::zip(*c1, **c2)
                            .map(|(c1, c2)| c1.abs_diff(c2))
                            .sum::<u8>()
                            == 1
                    })
                    .count()
            })
            .sum::<usize>()
}

fn parse() -> impl Iterator<Item = Coords> {
    INPUT.lines().map(|line| {
        let mut coords = line.split(',');
        std::array::from_fn(|_| coords.next().unwrap().parse().unwrap())
    })
}
