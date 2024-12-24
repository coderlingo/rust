use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Guess the number!");
    println!("Please input your number!");
    let mut guess = String::new();
        
    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        println!("The number entered is {}", guess);
        let guess:u32 = guess.trim().parse().expect("Please type a number");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less than the secret number!"),
            Ordering::Greater => println!("Your guess is greater than the secret number!"),
            Ordering::Equal => {
                println!("You guessed the number!");
                break;
            },

        }
        println!("Guess again!");
    }
}
