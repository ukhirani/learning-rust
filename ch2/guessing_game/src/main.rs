extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // will give compiler warning if you don't write this

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); //this is called
                                                                               //shadowing in which you kind of redeclare the same variable instead of using another one

        let secret_number = rand::thread_rng().gen_range(1, 101); //inclusive on the
                                                                  //lower bound and exclusive on the upper bound

        println!("The secret number is : {}", secret_number);

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
