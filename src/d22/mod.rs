const MAP: &str = include_str!("map.txt");
const INSTRUCTIONS: &str = include_str!("instructions.txt");

type Coords = (usize, usize);

enum Tile {
    Open,
    Wall,
}

const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;

struct Map {
    rows: Vec<Vec<Tile>>,
    offsets: Vec<usize>,
    is_cube: bool,
}

impl Map {
    fn new(input: impl Iterator<Item = &'static str>, is_cube: bool) -> Self {
        let mut rows: Vec<Vec<Tile>> = Vec::new();
        let mut offsets: Vec<usize> = Vec::new();

        for line in input {
            let start = line.find(|c| c != ' ').unwrap();
            offsets.push(start);
            rows.push(Vec::from_iter(line.chars().skip(start).map(|c| match c {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => panic!("bad input"),
            })));
        }

        Self {
            rows,
            offsets,
            is_cube,
        }
    }

    fn make_move(&self, &(x, y): &Coords, d: u8) -> (Coords, u8) {
        match d {
            RIGHT => {
                if x + 1 >= self.offsets[y] + self.rows[y].len() {
                    self.wrap_right(y)
                } else {
                    ((x + 1, y), RIGHT)
                }
            }
            LEFT => {
                if x == self.offsets[y] {
                    self.wrap_left(y)
                } else {
                    ((x - 1, y), LEFT)
                }
            }
            DOWN => {
                if y + 1 == self.rows.len() || !self.is_x_non_empty(&(x, y + 1)) {
                    self.wrap_down(x)
                } else {
                    ((x, y + 1), DOWN)
                }
            }
            UP => {
                if y == 0 || !self.is_x_non_empty(&(x, y - 1)) {
                    self.wrap_up(x)
                } else {
                    ((x, y - 1), UP)
                }
            }
            _ => panic!("bad direction"),
        }
    }

    /*
       .aa.bb
       c  |  d
       c  |  d
       .--.ee
       f  e
       f  e
    .ff.--.
    c  |  d
    c  |  d
    .--.gg.
    a  g
    a  g
    .bb.
    */

    fn wrap_right(&self, y: usize) -> (Coords, u8) {
        if self.is_cube {
            match y / 50 {
                0 => ((99, 149 - y), LEFT),  //d
                1 => ((50 + y, 49), UP),     //e
                2 => ((149, 149 - y), LEFT), //d
                3 => ((y - 100, 149), UP),   //g
                _ => panic!("bad coords"),
            }
        } else {
            ((self.offsets[y], y), RIGHT)
        }
    }

    fn wrap_left(&self, y: usize) -> (Coords, u8) {
        if self.is_cube {
            match y / 50 {
                0 => ((0, 149 - y), RIGHT),  //c
                1 => ((y - 50, 100), DOWN),  //f
                2 => ((50, 149 - y), RIGHT), //c
                3 => ((y - 100, 0), DOWN),   //a
                _ => panic!("bad coords"),
            }
        } else {
            ((self.offsets[y] + self.rows[y].len() - 1, y), LEFT)
        }
    }

    fn wrap_down(&self, x: usize) -> (Coords, u8) {
        if self.is_cube {
            match x / 50 {
                0 => ((100 + x, 0), DOWN),  //b
                1 => ((49, 100 + x), LEFT), //g
                2 => ((99, x - 50), LEFT),  //e
                _ => panic!("bad coords"),
            }
        } else {
            (
                (
                    x,
                    (0..self.rows.len())
                        .find(|y| self.is_x_non_empty(&(x, *y)))
                        .unwrap(),
                ),
                DOWN,
            )
        }
    }

    fn wrap_up(&self, x: usize) -> (Coords, u8) {
        if self.is_cube {
            match x / 50 {
                0 => ((50, x + 50), RIGHT), //f
                1 => ((0, 100 + x), RIGHT), //a
                2 => ((x - 100, 199), UP),  //b
                _ => panic!("bad coords"),
            }
        } else {
            (
                (
                    x,
                    (0..self.rows.len())
                        .rev()
                        .find(|y| self.is_x_non_empty(&(x, *y)))
                        .unwrap(),
                ),
                UP,
            )
        }
    }

    fn is_x_non_empty(&self, &(x, y): &Coords) -> bool {
        (self.offsets[y]..self.offsets[y] + self.rows[y].len()).contains(&x)
    }

    fn is_wall(&self, &(x, y): &Coords) -> bool {
        matches!(self.rows[y][x - self.offsets[y]], Tile::Wall)
    }
}

pub fn solve01() -> usize {
    solve(Map::new(MAP.lines(), false))
}

pub fn solve02() -> usize {
    solve(Map::new(MAP.lines(), true))
}

fn solve(map: Map) -> usize {
    let mut coords = (
        map.offsets[0]
            + map.rows[0]
                .iter()
                .position(|t| matches!(*t, Tile::Open))
                .unwrap(),
        0usize,
    );
    let mut d = RIGHT;

    let mut chars = INSTRUCTIONS.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            'R' => d = (d + 1) % 4,
            'L' => d = (d + 3) % 4,
            _ => {
                let mut number_as_string: String = c.into();
                while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                    number_as_string.push(c);
                }
                let number: usize = number_as_string.parse().unwrap();
                for _ in 0..number {
                    let (new_coords, new_direction) = map.make_move(&coords, d);
                    if map.is_wall(&new_coords) {
                        break;
                    } else {
                        coords = new_coords;
                        d = new_direction;
                    }
                }
            }
        }
    }

    1000 * (coords.1 + 1) + 4 * (coords.0 + 1) + d as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrap_cube() {
        let map = Map::new(MAP.lines(), true);
        // RIGHT
        // d
        assert_eq!(((99, 149), LEFT), map.make_move(&(149, 0), RIGHT));
        assert_eq!(((99, 100), LEFT), map.make_move(&(149, 49), RIGHT));
        // e
        assert_eq!(((100, 49), UP), map.make_move(&(99, 50), RIGHT));
        assert_eq!(((149, 49), UP), map.make_move(&(99, 99), RIGHT));
        // d
        assert_eq!(((149, 49), LEFT), map.make_move(&(99, 100), RIGHT));
        assert_eq!(((149, 0), LEFT), map.make_move(&(99, 149), RIGHT));
        // g
        assert_eq!(((50, 149), UP), map.make_move(&(49, 150), RIGHT));
        assert_eq!(((99, 149), UP), map.make_move(&(49, 199), RIGHT));

        //LEFT
        // c
        assert_eq!(((0, 149), RIGHT), map.make_move(&(50, 0), LEFT));
        assert_eq!(((0, 100), RIGHT), map.make_move(&(50, 49), LEFT));
        // f
        assert_eq!(((0, 100), DOWN), map.make_move(&(50, 50), LEFT));
        assert_eq!(((49, 100), DOWN), map.make_move(&(50, 99), LEFT));
        // c
        assert_eq!(((50, 49), RIGHT), map.make_move(&(0, 100), LEFT));
        assert_eq!(((50, 0), RIGHT), map.make_move(&(0, 149), LEFT));
        // a
        assert_eq!(((50, 0), DOWN), map.make_move(&(0, 150), LEFT));
        assert_eq!(((99, 0), DOWN), map.make_move(&(0, 199), LEFT));
    }
}
