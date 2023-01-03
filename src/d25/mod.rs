const INPUT: &str = include_str!("input.txt");

pub fn solve01() -> String {
    let sum: i64 = INPUT.lines().map(to_decimal).sum();
    to_snafu(sum)
}

fn to_snafu(d: i64) -> String {
    let top_expotent = (d as f64).log(5.0) as u32;
    let mut components: Vec<i64> = Vec::new();
    let mut remainder = d;
    for i in (0..=top_expotent).rev() {
        let component = remainder / 5_i64.pow(i);
        components.push(component);
        remainder -= component * 5_i64.pow(i);
    }

    let mut chars: Vec<char> = Vec::new();
    let mut carry_one = false;
    for component in components.into_iter().rev() {
        let c: char;
        (c, carry_one) = match component + i64::from(carry_one) {
            0 => ('0', false),
            1 => ('1', false),
            2 => ('2', false),
            3 => ('=', true),
            4 => ('-', true),
            5 => ('0', true),
            _ => unreachable!(),
        };
        chars.push(c);
    }

    if carry_one {
        chars.push('1');
    }

    String::from_iter(chars.into_iter().rev())
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().enumerate().fold(0, |acc, (i, c)| {
        acc + 5i64.pow((snafu.len() - i - 1) as u32)
            * match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("bad snafu"),
            }
    })
}
