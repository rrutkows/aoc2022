use std::{cmp::Ordering, iter::Peekable};

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Clone)]
enum PacketItem {
    Integer(i32),
    List(Vec<PacketItem>),
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use PacketItem::*;
        match self {
            Integer(i) => match other {
                Integer(i_other) => i.cmp(i_other),
                // Convert self to list
                List(_) => List(vec![Integer(*i)]).cmp(other),
            },
            List(l) => match other {
                //Convert other to list
                Integer(i_other) => self.cmp(&List(vec![Integer(*i_other)])),
                List(l_other) => l
                    .iter()
                    .take(l_other.len())
                    .enumerate()
                    //The result of comparing lists is the result of comparing the first items,
                    //that are not equal...
                    .find_map(|(i, item)| match item.cmp(&l_other[i]) {
                        Ordering::Equal => None,
                        Ordering::Greater => Some(Ordering::Greater),
                        Ordering::Less => Some(Ordering::Less),
                    })
                    //... if no such item is found, the shorter list should be first
                    .unwrap_or_else(|| l.len().cmp(&l_other.len())),
            },
        }
    }
}

pub fn solve01() -> i32 {
    let mut lines = INPUT.lines();
    let mut i = 0;
    let mut sum = 0;
    while let Some(line) = lines.next() {
        i += 1;
        let p1 = parse(line);
        let p2 = parse(lines.next().unwrap());
        if p1 < p2 {
            sum += i;
        }
        lines.next();
    }

    sum
}

pub fn solve02() -> usize {
    let mut lines = INPUT.lines();
    let mut packets: Vec<PacketItem> = Vec::new();
    while let Some(line) = lines.next() {
        packets.push(parse(line));
        packets.push(parse(lines.next().unwrap()));
        lines.next();
    }

    let d1 = PacketItem::List(vec![PacketItem::Integer(2)]);
    let d2 = PacketItem::List(vec![PacketItem::Integer(6)]);
    packets.push(d1.clone());
    packets.push(d2.clone());

    packets.sort();

    (1 + packets.binary_search(&d1).unwrap()) * (1 + packets.binary_search(&d2).unwrap())
}

fn parse(s: &str) -> PacketItem {
    parse_item(&mut s.chars().peekable())
}

fn parse_item<T: Iterator<Item = char>>(s: &mut Peekable<T>) -> PacketItem {
    let c = s.next().unwrap();
    match c {
        '[' => parse_list(s),
        _ => {
            let mut item: String = c.into();
            while let Some(c_item) = s.next_if(|&c| c.is_ascii_digit()) {
                item.push(c_item);
            }
            PacketItem::Integer(item.parse::<i32>().unwrap())
        }
    }
}

fn parse_list<T: Iterator<Item = char>>(s: &mut Peekable<T>) -> PacketItem {
    let mut list: Vec<PacketItem> = Vec::new();
    loop {
        if let Some(c) = s.next_if(|&c| c == ',' || c == ']') {
            match c {
                ',' => {}
                ']' => break,
                _ => unreachable!(),
            }
        }
        list.push(parse_item(s));
    }
    PacketItem::List(list)
}
