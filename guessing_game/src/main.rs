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

    println!("Creating a linked List now");
    let mut input = String::new();
    let mut size:u32 = match io::stdin().read_line(&mut input) {
        Ok(n) => {
           input.trim().parse().expect("Should be a number")
        },
        Err(error) => panic!("error: {error}"),
    };
    let mut head:Node<'_> = Node::create_node();
    let mut prev:&Node<'_> = head;
    for i in (0..size) {
        let mut n:Node = Node {
            val: i,
            next: None,
        };
        match head {
            None => {
                head = n;
                prev = Some(&n);
            },
            _ => {
                println!("Head is initialized");
                *prev.next = Some(&n);
            }
        };
        prev = &n;
    }

}
struct Node<'a> { 
    val: u32,
    next: Option<&'a Node<'a>>,
    is_empty: bool,
}

impl Node<'_> {
    fn create_node() -> Self{
        Self{
            next: None,
            val: 1,
            is_empty: true,
        }
    }
    fn update_val(&mut self, val:u32) {
        self.val = val;
        self.is_empty = false;
    }
}

