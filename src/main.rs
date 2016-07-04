extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("I'm thinking of a goddamned number between 1 and 100 damnit.");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	loop {
		println!("What's your damn guess?");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Couldn't read your damn number.");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("I said type a damn number!");
				continue;
			},
		};
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too damn low."),
			Ordering::Greater => println!("Too damn high."),
			Ordering::Equal => {
				println!("Damn right.");
				break;
			},
		}
	}
}