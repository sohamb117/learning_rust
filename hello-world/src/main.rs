use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io;

fn main() {
    println!("Hello, world!");

    let num = 10;

    let mut age: i32 = 40;

    let is_it_true: bool = true;

    let let_x: char = 'x';

    println!("I am {} years old", age);

    let (f_name, l_name) = ("Soham", "Bhattacharya");

    println!("it is {0} that {1} is {0}", is_it_true, let_x);

    println!("{:.2}",1.234);

    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    println!("{ten:>ws$}", ten = 10, ws = 5);

    println!("{ten:>0ws$}", ten = 10, ws = 5);

    let age_old = 22;

    if(age_old == 5) {
        println!("Go to Kindergarten");
    } else if(age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", (age_old - 5));
    } else if(age_old <= 25) &&(age_old > 19) {
        println!("Go to College");
    } else {
        println!("Do what you want");
    }

    let can_vote = if (age_old >= 18) {true} else {false};

    let mut x = 0;
    loop {
        if((x % 2) == 0){
            println!("{}", x);
            x += 1;

            continue;
        }
        if(x > 10) {
            break;
        }
        x+=1;
    }
    let mut y = 1;
    while y <= 10 {
        println!("{}",y);
        y+=1;
    }
    for z in 1..10 {
        println!("FOR : {}", z);
    }

    let rand_string = "I am a random string";

    println!("Length : {}", rand_string.len());
    
    let (first, second) = rand_string.split_at(6);
    println!("First: {}", first);
    println!("Second: {}", second);
    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();
    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();


    }
} 
