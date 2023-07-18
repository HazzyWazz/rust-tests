// Body Mass Index (BMI) is a person’s weight in kilograms (or pounds) divided by the square of height in meters (or feet).
// --CDC

use std::io::{self, Write, Result}; // import std::io

// Displays a message to `STDOUT`, then return a line from `STDIN` with whitespace trimmed from both sides
fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    // ensure the above is outputted before reading input, returns the error if there's an error flushing
    io::stdout().flush()?;

    let mut input = String::new();
    // read line from stdin into input, returning the error if there's an error reading the line
    io::stdin().read_line(&mut input)?;

    // remove whitespace from the input
    Ok(input.trim().to_string())
}

// ↑ standard issue Copy Paste Programming

fn main() {
	// parse user's input height and weight
	let height: f64 = prompt("Input your weight in centimiters: ").expect("Error reading STDIN").parse().expect("Error parsing height");
	let weight: f64 = prompt("Input your weight in kilograms: ").expect("Error reading STDIN").parse().expect("Error parsing weight");

	// create float bmi and calculate BMI from Kg and meters
	let bmi: f64 = weight / (height/100.0).powf(2.0);

	// print bmi, format to print only up to 2 decimals
	println!("Your BMI is {:.2}", bmi);

}