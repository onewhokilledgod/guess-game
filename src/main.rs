use std::io;

fn main() {
	println!("Guess the number!");
	println!("lease input your guess.");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess).expect("error");

	print!("you gessed: {guess}")
}
