const INPUT: &str = include_str!("input.txt");

const DIGITS: [char; 5] = ['0', '1', '2', '=', '-'];

pub fn solve01() -> String {
    let sum: i64 = INPUT.lines().map(to_decimal).sum();
    to_snafu(sum)
}

fn to_snafu(d: i64) -> String {
    let mut chars: Vec<char> = Vec::new();
    let mut d = d;
    loop {
        let digit = (d % 5) as usize;
        chars.push(DIGITS[digit]);
        d = d / 5 + i64::from(digit > 2);
        if d == 0 {
            break;
        }
    }

    String::from_iter(chars.into_iter().rev())
}

fn to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .fold((0, 1), |(acc, base), c| {
            (
                acc + base
                    * match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!("bad snafu"),
                    },
                base * 5,
            )
        })
        .0
}
