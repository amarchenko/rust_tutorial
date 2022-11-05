#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random num: {}", random_num);
}

fn sample4_main() {
    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    let num_2: f64 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    println!("f64: {}", num_2 + 0.111111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
    num_3 += 1;
}

fn sample3_main() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn sample2_main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I am {} and I want ${}", age, ONE_MIL);
}

fn sample1_main() {
    println!("Waht is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    let error: &str = "Didn't receive input";
    io::stdin().read_line(&mut name)
        .unwrap();

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
