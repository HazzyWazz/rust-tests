// hell yeah it's competition again!
use std::collections::HashMap;

fn main() {
	// let scores = [45, 32, 28, 30, 13, 25, 17, 36, 40, 25];
	// let people = ["Agyalá Gyula", "Békés Csaba", "Cicam Ica", "Futó Rózsa", "Kó Laci", "Meta Flóra", "Papl Andi", "Ultra Viola", "Vizi Lóránt", "Zok Niki"];
	let scores = HashMap::from([
		("Agyalá Gyula", 45),
		("Békés Csaba", 32),
		("Cicam Ica", 28),
		("Futó Rózsa", 30),
		("Kó Laci", 13),
		("Meta Flóra", 25),
		("Papl Andi", 17),
		("Ultra Viola", 36),
		("Vizi Lóránt", 40),
		("Zok Niki", 25)
	]);

	let test = scores
		.iter()
		.max_by_key(|&(_, x)| *x)
		.unwrap();

	println!("{:?}", test);
}
