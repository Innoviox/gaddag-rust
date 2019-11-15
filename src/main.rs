use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let answer = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("What is your guess? ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("You win");
                break
            }
        }
    }
}