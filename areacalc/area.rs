// input: side a, b
// output: calculate perimiter P and area A
// only for squares and rectangles :p

use std::io::{self, Write};

fn Square(a: u32, b: u32) -> bool {
	a == b
}

fn CalcPandA(a: u32, b: u32, Sqaureness: bool) { // parameters are the four sides
	let Perimiter = a + b; // calculate perimiter
	let Area = a * b; // calculate area
	if Sqaureness {
		println!("Square\nPerimiter: {Perimiter}\nArea: {Area}");
	} else {
		println!("Rectangle\nPerimiter: {Perimiter}\nArea: {Area}");
	}
}

fn main() {
	let mut Sidea = String::new();
	print!("Length of side A (no units, positive numbers only) ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut Sidea).expect("Error reading from STDIN");

	let mut Sideb = String::new();
	print!("Length of side B (no units, positive numbers only) ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut Sideb).expect("Error reading from STDIN");

	let a: u32 = Sidea.trim().parse().expect("Unable to parse number");
	let b: u32 = Sideb.trim().parse().expect("Unable to parse number");

	CalcPandA(a, b, Square(a, b));
}