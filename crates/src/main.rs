use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Your guess: ");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Error while trying to read input!");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret) {
			Ordering::Less => println!("Higher"),
			Ordering::Greater => println!("Lower"),
			Ordering::Equal => {
				println!("You win! The answer was {secret}");
				break;
			}
		}
	}

}