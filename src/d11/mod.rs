use std::{
    ops::{Add, Mul},
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");

const REDUCER: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn get_result<T>(&self, arg1: &T, arg2: &T) -> T
    where
        for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Self::Add => arg1 + arg2,
            Self::Mul => arg1 * arg2,
        }
    }
}

#[derive(Debug)]
enum OperationArgument<T> {
    Const(T),
    OldValue,
}

#[derive(Debug)]
struct Operation<T> {
    operator: Operator,
    argument: OperationArgument<T>,
}

impl<T> Operation<T> {
    fn perform(&self, old_value: &T) -> T
    where
        for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
    {
        match &self.argument {
            OperationArgument::Const(c) => self.operator.get_result(old_value, c),
            OperationArgument::OldValue => self.operator.get_result(old_value, old_value),
        }
    }
}

#[derive(Debug)]
struct ThrowTarget {
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    holding: Vec<u64>,
    operation: Operation<u64>,
    condition: i32,
    throw_target: ThrowTarget,
    worry_reducer: i32,
    counter: u64,
}

impl Monkey {
    fn throw(&mut self) -> impl Iterator<Item = (u64, usize)> + '_ {
        self.holding.drain(0..).map(|item| {
            self.counter += 1;
            let value = self.operation.perform(&item) / self.worry_reducer as u64;
            let target = match value % self.condition as u64 {
                0 => self.throw_target.if_true,
                _ => self.throw_target.if_false,
            };
            (value % REDUCER, target)
        })
    }
}

pub fn solve(round_count: usize, worry_reducer: i32) -> u64 {
    let mut monkeys = parse(worry_reducer);
    let len = monkeys.len();
    //dbg!(monkeys);

    for _ in 0..round_count {
        for i in 0..len {
            #[allow(clippy::needless_collect)]
            let thrown: Vec<(u64, usize)> = monkeys[i].throw().collect();
            for (value, target) in thrown.into_iter() {
                monkeys[target].holding.push(value);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.counter.cmp(&m1.counter));
    monkeys.into_iter().take(2).map(|m| m.counter).product()
}

fn parse(worry_reducer: i32) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut lines = INPUT.lines();
    while lines.next().is_some() {
        let holding: Vec<u64> = lines
            .next()
            .unwrap()
            .split_once(": ")
            .map(|(_, list)| {
                list.split(", ")
                    .map(|s| u64::from_str(s).unwrap())
                    .collect()
            })
            .unwrap();
        let operation = lines
            .next()
            .unwrap()
            .rsplit_once(' ')
            .map(|(op, arg)| Operation {
                operator: if op.ends_with('*') {
                    Operator::Mul
                } else {
                    Operator::Add
                },
                argument: if arg == "old" {
                    OperationArgument::OldValue
                } else {
                    OperationArgument::Const(u64::from_str(arg).unwrap())
                },
            })
            .unwrap();
        monkeys.push(Monkey {
            holding,
            operation,
            condition: read_number(&mut lines),
            throw_target: ThrowTarget {
                if_true: read_number(&mut lines),
                if_false: read_number(&mut lines),
            },
            worry_reducer,
            counter: 0,
        });

        lines.next(); //empty line
    }

    monkeys
}

fn read_number<'a, TInput, TOutput>(input: &mut TInput) -> TOutput
where
    TInput: Iterator<Item = &'a str>,
    TOutput: FromStr,
    TOutput::Err: std::fmt::Debug,
{
    input
        .next()
        .unwrap()
        .rsplit_once(' ')
        .map(|(_, c)| TOutput::from_str(c).unwrap())
        .unwrap()
}
