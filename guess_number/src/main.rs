extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("let's input hit number, random number range of 1 to 10");
    run();
}

fn run() {
    let secret = rand::thread_rng().gen_range(1..10);
    let guess = input();
    if !is_match(secret, guess) {
        run();
    }
}

fn input() -> u32 {
    let mut guess = String::new(); // mutable string
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess.trim().parse().expect("plz, number");
}

fn is_match(secret: u32, guess: u32) -> bool {
    let mut is = false;
    match guess.cmp(&secret) {
        Ordering::Less => println!("Less!"),
        Ordering::Greater => println!("Greater!"),
        Ordering::Equal => {
            is = true;
        }
    }
    return is;
}
