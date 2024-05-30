use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing Game!");

    let number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("{}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&number) {
            Ordering::Equal => {
                println!("U guessed it {number}");
                break;
            }
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        }
    }
}
