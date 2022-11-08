#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
}

fn sample14_main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.chars() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}

fn sample13_main() {
    let my_tuple: (u8, String, f64) = (38, "Alex".to_string(), 50_000_000.0);

    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn sample12_main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;

    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}

fn sample11_main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}

fn sample10_main() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}

fn sample9_main() {
    let age2 = 19;
    match age2 {
        1..=19 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Importatn Birthday"),
    };
}

fn sample7_main() {
    let mut my_age = 14;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote : {}", can_vote);
}

fn sample6_main() {
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Brirthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }
}

fn sample5_main() {
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
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I am {} and I want ${}", age, ONE_MIL);
}

fn sample1_main() {
    println!("Waht is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    let error: &str = "Didn't receive input";
    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
