#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Waht is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    let error: &str = "Didn't receive input";
    io::stdin().read_line(&mut name)
        .unwrap();

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
