// input: side a, b
// output: calculate perimiter P and area A
// only for squares and rectangles :p

use std::io::{self, Write};  //import io

fn Square(a: u32, b: u32) -> bool {
	a == b  // function to check if side A and B are the same 
}

fn CalcPandA(a: u32, b: u32, Sqaureness: bool) { // parameters are the four sides
	let Perimiter = a + b; // calculate perimiter
	let Area = a * b; // calculate area
	if Sqaureness {  // print it
		println!("Square\nPerimiter: {Perimiter}\nArea: {Area}");
	} else {
		println!("Rectangle\nPerimiter: {Perimiter}\nArea: {Area}");
	}
}

fn main() {
	let mut Sidea = String::new();  // create new empty mutable string
	print!("Length of side A (no units, positive numbers only) ");  // Ask for side A in STDOUT
	let _ = io::stdout().flush();  // flush
	io::stdin().read_line(&mut Sidea).expect("Error reading from STDIN");  // take input from STDIN


	// same here for side B
	let mut Sideb = String::new();
	print!("Length of side B (no units, positive numbers only) ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut Sideb).expect("Error reading from STDIN");

	// parse ints from side var strings
	let a: u32 = Sidea.trim().parse().expect("Unable to parse number");
	let b: u32 = Sideb.trim().parse().expect("Unable to parse number");

	CalcPandA(a, b, Square(a, b)); // calcualte the perimiter and area
}