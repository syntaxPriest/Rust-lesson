#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::stdin, mem};

fn for_loops(){
    let mut x = 1;
    
    let country_code = 2000;
    let country = match country_code {
        44 => "UK",
        234 => "NG",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("The country code with {}, is {}", country_code, country);
}

enum State {
    Locked,
    Failed,
    Unlocked
}

fn password_guess(){
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    let mut try_count = 0;
    let try_max = 5;


    println!("Enter a number and click enter:");
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                try_count += 1;
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    },
                    Err(_) => {
                        continue;
                    }
                }

                if code == entry{
                    state = State::Unlocked;
                }else if !code.starts_with(&entry){
                    state = State::Failed
                }
            },
            State::Failed => {
                if try_count < try_max {
                    println!(
                        "Wrong input, Try again!"
                    );
                    entry.clear();
                    state = State::Locked;
                    continue;
                }else {
                    println!("Sorry, Maximum trial reached!");
                    return;
                }
            },
            State::Unlocked => {
                println!("Unlocked Successfully!");
                return
            }
        }
    }
}

fn sum_products(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples(){
    let x = 4;
    let y = 5;
    let sp = sum_products(x,y);
    let (a, b) = sp;
    println!("{0} + {1} = {2}. And {0} * {1} = {3}", x, y, a, b)
}

struct User {
    name: String,
    age: u8,
    occupation: String,
}

fn struct_fn() {
    let user: User = User {
        name: String::from("Daniel Adewale"),
        occupation: String::from("Software Engineer"),
        age: 22
    };

    println!("{}, is a {}, and he is {} years old", user.name, user.occupation, user.age);
}

fn option_match(){
    let x = 5.0;
    let y = 2.0;

    let result = if y != 0.0 {Some(x/y)} else { None };

    match result {
        Some(r) => {
            println!("Result is {}", r)
        }
        None => {
            println!("Can't divide by zero")
        }
    }
}



fn main() {
    let a : u8 = 255;
    println!("a is {}", a);

    let mut b : i8 = 123;
    println!("b is {} before", b);
    b = 0;
    println!("b is {} after", b);
    
    let c = 800000000;
    println!("c is {}, and it takes up {} bytes", c, mem::size_of_val(&c));

    let b = 2.5;
    let b_to_pie = f64::powf(b, std::f64::consts::PI);

    println!("b_to_pie is {}", b_to_pie);

    // for_loops();
    // password_guess();
    struct_fn();
    option_match();
    tuples();
}

