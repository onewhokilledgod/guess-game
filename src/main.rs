use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
	let secret_number = rand::rng().random_range(1..=100);
	let mut guess = String::new();

	println!("Guess the number!");
	println!("Please input your guess.");

	io::stdin().read_line(&mut guess).expect("error");
	let guess: u32 = guess.trim().parse().expect("Please type a number!");

	match guess.cmp(&secret_number) {
		Ordering::Greater => println!("Too big!"),
		Ordering::Less => println!("Too small!"),
		Ordering::Equal => println!("You win!"),
	}

	println!("You guessed: {guess}");
}
