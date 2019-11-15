use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn thingy() -> u32 {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}

fn main() {
    let x = thingy();
    println!("The value of x is: {}", x);
}