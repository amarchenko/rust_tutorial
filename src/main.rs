#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn sum_list(list: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in list {
        sum += *i;
    }
    return sum;
}

fn main() {
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_2(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

fn sample18_main() {
    get_sum(4, 5);
    println!("{}", get_sum2(4, 5));

    let (val_1, val_2 ) = get_2(3);
    println!("Nums : {}, {}", val_1, val_2);
}

fn sample17_main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &mut vec2 {
        println!("{}", i);
    }
    println!("Vec length : {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

}

fn sample16_main() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates Monday."),
        Days::Tuesday => println!("Donat day."),
        Days::Wednesday => println!("Hump day."),
        Days::Thursday => println!("Pay day."),
        Days::Friday => println!("Almost weekend."),
        Days::Saturday => println!("Weekend."),
        Days::Sunday => println!("Weekend."),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}

fn sample15_main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("int u32: {}", int3_u32);
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

    let st8 = String::from("Just some words");
    for char in st8.bytes() {
        println!("Byte: {}", char);
    }
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
