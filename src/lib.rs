use std::{fmt::Display, time::Instant};

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;

pub fn run<T, F>(f: F)
where
    T: Display,
    F: Fn() -> T,
{
    let start = Instant::now();
    println!("{} {:?}", f(), start.elapsed());
}
