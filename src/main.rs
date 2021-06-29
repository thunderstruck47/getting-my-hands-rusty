use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);

    println!("Welcome!");
    println!("You are about to take part in a guessing game. (I hope that you won't look at the memory at runtime.)");
    println!("Please enter a number from 1 to 100:");

    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Try a bigger number"),
            Ordering::Equal => {
                println!("You win! The number is: {}", secret);
                break;
            },
            Ordering::Greater => println!("Try a lower number")
        }
    }
}
