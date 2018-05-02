extern crate rand;

use std::io;
use std::cmp::Ordering;

use rand::Rng;


fn main() {
    const TRIES: i32 = 10;

    println!("Program has thought of a number between 1 and 100");
    println!("You have {} tries to guess that number", TRIES);

    let mut rng = rand::thread_rng();
    let program_number: i32 = rng.gen_range(1, 101);


    for i in 0..TRIES {
        let guess = read_user_guess();

        match guess.cmp(&program_number) {
            Ordering::Less => println!("Too small! {} tries left", TRIES - 1 - i),
            Ordering::Greater => println!("Too big! {} tries left", TRIES - 1 - i),
            Ordering::Equal => {
                println!("You win!");
                return;
            },
        }
    }

    println!("You are out of tries! The number was {}", program_number);
}


fn read_user_guess() -> i32 {
    loop {
        println!("Input a number between 1 and 100:");

        let mut sguess = String::new();
        io::stdin().read_line(&mut sguess)
            .expect("Failed to read line");

        let guess: i32 = match sguess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input: {}", sguess);
                continue;
            },
        };

        if !(guess >= 0 && guess < 100) {
            println!("Number {} is not from 1..100 range", guess);
            continue;
        }

        return guess;
    }
}
