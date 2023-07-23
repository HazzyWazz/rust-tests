// use std::io::{self, Write, Result}; // import std::io
use std::io::{self, Write}; // import std::io

fn maxindex(list: &[u64]) -> usize {  // get index of highest value from an array full of ints, return value as usize (it is usize)
	let m = list
		.iter() // iterate through array
		.max()  // get highest value
		.expect("Error while trying to get highest value!"); // error if there's an issue
	let i_of_high = list
		.iter() // iterate through array
		.position(|&x| x == *m) // get index of first element matching max value
		.unwrap(); // error silently?
	i_of_high // return index of max (usize)
}

// Displays a message to `STDOUT`, then return a line from `STDIN` with whitespace trimmed from both sides
fn prompt(message: &str) ->  std::io::Result<String> {
	print!("{message}");
	// ensure the above is outputted before reading input, returns the error if there's an error flushing
	io::stdout().flush()?;

	let mut input = String::new();
	// read line from stdin into input, returning the error if there's an error reading the line
	io::stdin().read_line(&mut input)?;

	// remove whitespace from the input
	Ok(input.trim().to_string())
}

fn search(list: &[&str], name: String) -> Result<usize, &'static str> {
	if list.contains(&&*name) {
		let index = list
			.iter()
			.position(|&x| *x == *name)
			.expect("Error while trying to get index of name!");
		Ok(index)
	} else {
		Err("{name} does not exist in the array!")
	}
}


fn main() {
	let scores = [45, 32, 28, 30, 13, 25, 17, 36, 40, 25];
	let people = ["Agyalá Gyula", "Békés Csaba", "Cicam Ica", "Futó Rózsa", "Kó Laci", "Meta Flóra", "Papl Andi", "Ultra Viola", "Vizi Lóránt", "Zok Niki"];

	// 1. print highest score (45)
	let max = scores
		.iter() // go through array
		.max()  // get highest value
		.expect("Error while trying to get highest value!"); // error if there's an issue
	println!("Highest score: {}", max);  // print it

	// 2. print average (29.1)
	let length: u64 = scores
		.len() // get the length of array 'scores'
		.try_into()  // try turning it into a u64 integer
		.unwrap();  // error silently?

	let total: u64 = scores.iter().sum();  // iterate through and sum all values of array 'score'
	let average = total as f64 / length as f64;  // average them out from total value and length
	println!("Average score: {}", average); // print it

	// 3. get name of person with highest score (Agyalá Gyula: 45)
	let index_of_highest = scores
		.iter() // iterate through array
		.position(|&x| x == *max) // get index of first element matching max value
		.unwrap(); // error silently?
	println!("Highest score {} achieved by {}", scores[index_of_highest], people[index_of_highest]); // print it

	// 4. same with func
	let msc = maxindex(&scores); // declare 'msc' as returned value from maxindex function
	println!("Highest score {} achieved by {}", scores[msc], people[msc]); // print the values of the arrays 'scores' and 'people'

	// 5. print names of people with scores higher than X (30)
	scores
		.into_iter() // turn into Iterator type
		.filter(|&score| score > 30) // filter for all values above 30
		.for_each(|score| { // for each value above 30 ...
			let scoreindex = scores
				.iter() // ... iterate through list
				.position(|&x| x == score) // get index of first element matching that value
				.unwrap(); // error silently?
			println!("{} got a score of {} (over 30)", people[scoreindex], scores[scoreindex]); // print it
		});

	let query = prompt("Search: ");

	if query.is_ok() {
		let aquery = query.unwrap();
		let ind = search(&people, aquery.clone()).unwrap();
		println!("{aquery} achieved a score of {}", scores[ind]);
	} else {
		println!("{:?}", query.unwrap());
	}

}