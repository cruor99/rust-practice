extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut attempts: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {

            Ordering::Less => {
                increment(&mut attempts);
                println!("Too small!")
            },
            Ordering::Greater => {
                increment(&mut attempts);
                println!("Too big!");
            },
            Ordering::Equal => {
                increment(&mut attempts);
                println!("You win!");
                println!("{}", attempts);
                break;
            }
        }
    }
}

fn increment(r: &mut u32){
    *r = *r +1;
}
